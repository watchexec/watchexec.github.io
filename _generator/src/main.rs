use std::{convert::TryInto, path::PathBuf};

use chrono::Utc;
use color_eyre::{eyre::eyre, Result};
use config::{App, Cat, Config, Format, SumAlgo};
use globset::Glob;
use isahc::{AsyncReadResponseExt, HttpClientBuilder, config::{Configurable, RedirectPolicy}};
use semver::Version;
use structopt::StructOpt;
use url::Url;

mod config;

#[derive(Clone, Debug, StructOpt)]
enum Mode {
	Lint {
		#[structopt(short, long = "config")]
		config_file: PathBuf,

		#[structopt(long)]
		print: bool,
	},

	ReleaseMeta {
		#[structopt(short, long = "config")]
		config_file: PathBuf,

		#[structopt(short, long)]
		app: String,

		#[structopt(short, long)]
		version: Option<String>,
	},
}

#[async_std::main]
async fn main() -> Result<()> {
	match Mode::from_args() {
		Mode::Lint { config_file, print } => {
			let config = Config::load(&config_file).await?;
			if print {
				println!("{:#?}", config);
			}
		}

		Mode::ReleaseMeta {
			config_file,
			app,
			version,
		} => {
			let version = version
				.map(|v| Version::parse(&v))
				.transpose()?
				.expect("TODO: get latest version");

			let config = Config::load(&config_file).await?;
			let app = config.app(&app, &version)?;

			let release = octocrab::instance()
				.repos(&app.repo.owner, &app.repo.repo)
				.releases()
				.get_by_tag(&app.tag(&version)?)
				.await?;

			let mut downloads = Vec::new();
			let mut sums = Vec::new();
			'assets: for asset in &release.assets {
				let url = &asset.browser_download_url;
				let filename = url
					.path_segments()
					.and_then(|s| s.last())
					.ok_or_else(|| eyre!("download url is malformed: {}", url))?;

				for (algo, sum) in &config.checksums {
					if filename.starts_with(&sum.filename) {
						sums.push((*algo, url, filename));
						continue 'assets;
					}
				}

				match Download::new(&config, url, filename, asset.size.try_into()?) {
					Ok(dl) => downloads.push(dl),
					Err(err) => eprintln!("warning: {}", err),
				}
			}

			let known_sign_glob = Glob::new(&format!(
				"{{{}}}.{{{}}}.minisig",
				config
					.checksums
					.iter()
					.map(|(_, c)| c.filename.as_str())
					.collect::<Vec<&str>>()
					.join(","),
				config
					.maintainers
					.iter()
					.map(|m| m.username.as_str())
					.chain(std::iter::once("auto"))
					.collect::<Vec<&str>>()
					.join(","),
			))?
			.compile_matcher();

			sums.sort_by_key(|(algo, _, filename)| (*algo, filename.to_string()));
			let sums = sums
				.into_iter()
				.fold(SumFold::Init, |state, (algo, url, filename)| {
					if filename == config.checksums.get(&algo).unwrap().filename {
						state.add_sum(algo, url)
					} else if known_sign_glob.is_match(filename) {
						state.add_sign(&config, &app, url, filename)
					} else {
						eprintln!("unknown meta file: {}", filename);
						state
					}
				})
				.finish();

			let http = HttpClientBuilder::new()
				.redirect_policy(RedirectPolicy::Follow)
				.build()?;
			for sum in &sums {
				let sumfile = http.get_async(sum.url.to_string()).await?.text().await?;
				for line in sumfile.lines() {
					let mut parts = line.split_whitespace();

					let sumtext = match parts.next() {
						Some(s) => s,
						None => continue,
					};
					let sumfile = match parts.next() {
						Some(s) => s,
						None => continue,
					};

					for dl in &mut downloads {
						if dl.filename == sumfile {
							dl.sums.push(DownloadSum {
								algo: sum.algo,
								sum: sumtext.into(),
							});
						}
					}
				}
			}

			downloads.sort_by_key(|dl| (dl.cats.clone(), dl.format.short.clone()));
			serde_json::to_writer(std::io::stdout(), &serde_json::json!({
				"generated": Utc::now(),
				"version": version,
				"downloads": downloads,
				"sums": sums,
			}))?;
		}
	}

	return Ok(());
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Download {
	pub url: Url,
	pub filename: String,
	pub size: usize,
	pub format: Format,
	pub sums: Vec<DownloadSum>,
	pub cats: (String, String, Option<String>),
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DownloadSum {
	algo: SumAlgo,
	sum: String,
}

impl Download {
	pub fn new(config: &Config, url: &Url, filename: &str, size: usize) -> Result<Self> {
		let mut cats = Vec::new();
		for (glob, cat) in &config.triples {
			if glob.compile_matcher().is_match(filename) {
				cats.push(cat);
			}
		}

		let os = cats
			.iter()
			.find_map(|Cat { os, .. }| os.as_ref())
			.cloned()
			.ok_or_else(|| eyre!("no os found for: {}", filename))?;
		let arch = cats
			.iter()
			.find_map(|Cat { arch, .. }| arch.as_ref())
			.cloned()
			.ok_or_else(|| eyre!("no arch found for: {}", filename))?;
		let variant = cats
			.iter()
			.find_map(|Cat { variant, .. }| variant.as_ref())
			.cloned();

		Ok(Self {
			url: url.clone(),
			filename: filename.into(),
			size,
			format: config.match_format(&filename)?.clone(),
			sums: Vec::new(),
			cats: (os, arch, variant),
		})
	}
}

use serde::Serialize;

// typestate
#[derive(Debug, Clone)]
enum SumFold {
	Init,
	Current {
		acc: Vec<SignedSum>,
		algo: SumAlgo,
		url: Url,
		signs: Vec<Sign>,
	},
}

impl SumFold {
	fn add_sum(self, algo: SumAlgo, url: &Url) -> Self {
		let acc = match self {
			Self::Init => Vec::new(),
			Self::Current { mut acc, algo, url, signs } => {
				acc.push(SignedSum { algo, url, signs });
				acc
			}
		};

		Self::Current {
			acc,
			algo,
			url: url.clone(),
			signs: Vec::new(),
		}
	}

	fn add_sign(self, config: &Config, app: &App, sign_url: &Url, filename: &str) -> Self {
		let app_key_url = match &app.key_path {
			Some(kp) => Url::parse(&format!("https://github.com/{}/raw/-/{}", app.repo, kp.display())).unwrap(),
			None => return self,
		};

		match self {
			Self::Init => {
				eprintln!("lone signature without checksum (or bug!): {}", sign_url);
				Self::Init
			}
			Self::Current { acc, algo, url, mut signs } => {
				let user = filename.split('.').nth(1).expect("should be caught by glob (format)");
				let sign = if user == "auto" {
					Sign {
						sign_url: sign_url.clone(),
						key_url: app_key_url,
						name: "Automated signature".into(),
					}
				} else {
					let maint = config.maintainers.iter().find(|m| m.username == user).expect("should be caught by glob (user)");
					Sign {
						sign_url: sign_url.clone(),
						key_url: maint.key_url.clone(),
						name: format!("{}’s signature", maint.name),
					}
				};

				signs.push(sign);
				Self::Current { acc, algo, url, signs }
			}
		}
	}

	fn finish(self) -> Vec<SignedSum> {
		match self {
			Self::Init => Vec::new(),
			Self::Current {
				mut acc,
				algo,
				url,
				signs,
			} => {
				acc.push(SignedSum { algo, url, signs });
				acc
			}
		}
	}
}

#[derive(Clone, Debug, Serialize)]
pub struct SignedSum {
	pub algo: SumAlgo,
	pub url: Url,
	pub signs: Vec<Sign>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Sign {
	pub sign_url: Url,
	pub key_url: Url,
	pub name: String,
}
