use std::path::PathBuf;

use color_eyre::Result;
use config::Config;
use semver::Version;
use structopt::StructOpt;

mod config;
mod meta;
mod release;

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
			let meta = release::get_meta(config, app, version).await?;
			serde_json::to_writer(std::io::stdout(), &meta)?;
		}
	}

	return Ok(());
}
