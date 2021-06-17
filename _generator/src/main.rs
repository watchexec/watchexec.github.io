use std::path::PathBuf;

use async_std::{fs::{File, create_dir_all}, io::prelude::WriteExt};
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

			let dirpath = app.dir(&version);
			create_dir_all(&dirpath).await?;

			let meta = release::get_meta(config, app, version).await?;
			let bytes = serde_json::to_vec(&meta)?;

			let filepath = dirpath.join("meta.json");
			let mut file = File::create(&filepath).await?;
			file.write_all(&bytes).await?;

			eprintln!("wrote {} bytes to {}", bytes.len(), filepath.display());
		}
	}

	return Ok(());
}
