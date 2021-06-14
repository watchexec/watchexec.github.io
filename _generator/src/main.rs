use std::path::PathBuf;

use color_eyre::{eyre::eyre, Result};
use semver::Version;
use structopt::StructOpt;

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

			println!("{:?}", app);

			let release = octocrab::instance()
				.repos(&app.repo.owner, &app.repo.repo)
				.releases()
				.get_by_tag(&app.tag(&version)?)
				.await?;

			for asset in &release.assets {
				let url = &asset.browser_download_url;
				let filename = url
					.path_segments()
					.and_then(|s| s.last())
					.ok_or_else(|| eyre!("download url is malformed: {}", url))?;

				println!(
					"size={}\ttype={}\turl={}",
					asset.size,
					config.format_name(&filename),
					url
				);
			}
		}
	}

	return Ok(());
}
