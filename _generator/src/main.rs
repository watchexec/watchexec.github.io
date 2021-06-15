use std::{convert::TryInto, path::PathBuf};

use color_eyre::{eyre::eyre, Result};
use config::SumAlgo;
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

	ReleasePage {
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
			let config = config::Config::load(&config_file).await?;
			if print {
				println!("{:#?}", config);
			}
		}

		Mode::ReleasePage {
			config_file,
			app,
			version,
		} => {
			let version = version
				.map(|v| Version::parse(&v))
				.transpose()?
				.expect("TODO: get latest version");

			let config = config::Config::load(&config_file).await?;
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
						sums.push((*algo, filename));
						continue 'assets;
					}
				}

				downloads.push(Download {
					url: url.clone(),
					filename: filename.into(),
					size: asset.size.try_into()?,
					sums: Vec::new(),
				});
			}

			for dl in downloads {
				println!(
					"size={}\ttype={}\turl={}",
					dl.size,
					config.match_format(&dl.filename)?.short,
					dl.url
				);
			}
		}
	}

	return Ok(());
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Download {
	url: Url,
	filename: String,
	size: usize,
	sums: Vec<DownloadSum>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DownloadSum {
	algo: SumAlgo,
	hex: String,
}
