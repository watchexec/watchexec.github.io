use std::convert::TryInto;

use color_eyre::{eyre::eyre, Result};
use globset::Glob;
use isahc::{
	config::{Configurable, RedirectPolicy},
	AsyncReadResponseExt, HttpClientBuilder,
};
use semver::Version;
use url::Url;

use crate::config::{App, Config, SumAlgo};
use crate::meta::{Download, DownloadSum, Meta, Sign, SignedSum};

pub async fn get_meta(config: Config, app: App, version: Version) -> Result<Meta> {
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
	Ok(Meta::new(version, downloads, sums))
}

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
			Self::Current {
				mut acc,
				algo,
				url,
				signs,
			} => {
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
			Some(kp) => Url::parse(&format!(
				"https://github.com/{}/raw/-/{}",
				app.repo,
				kp.display()
			))
			.unwrap(),
			None => return self,
		};

		match self {
			Self::Init => {
				eprintln!("lone signature without checksum (or bug!): {}", sign_url);
				Self::Init
			}
			Self::Current {
				acc,
				algo,
				url,
				mut signs,
			} => {
				let user = filename
					.split('.')
					.nth(1)
					.expect("should be caught by glob (format)");
				let sign = if user == "auto" {
					Sign {
						sign_url: sign_url.clone(),
						key_url: app_key_url,
						name: "Automated signature".into(),
					}
				} else {
					let maint = config
						.maintainers
						.iter()
						.find(|m| m.username == user)
						.expect("should be caught by glob (user)");
					Sign {
						sign_url: sign_url.clone(),
						key_url: maint.key_url.clone(),
						name: format!("{}’s signature", maint.name),
					}
				};

				signs.push(sign);
				Self::Current {
					acc,
					algo,
					url,
					signs,
				}
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
