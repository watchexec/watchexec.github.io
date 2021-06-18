use std::path::PathBuf;

use async_std::{fs::{File, create_dir_all}, io::prelude::WriteExt};
use color_eyre::Result;
use config::Config;
use semver::Version;
use structopt::StructOpt;
use tera::{Context, Tera};

use crate::meta::Meta;

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

	ReleasePage {
		#[structopt(short, long = "config")]
		config_file: PathBuf,

		#[structopt(short, long = "meta")]
		meta_file: PathBuf,
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

		Mode::ReleasePage {
			config_file,
			meta_file,
		} => {
			let config = Config::load(&config_file).await?;
			let mut meta = Meta::load(&meta_file).await?;
			let app = config.app(&meta.app, &meta.version)?;

			// work around a limitation of Tera where the group_by filter
			// cannot group by a value being null (as it discards instead)
			for dl in &mut meta.downloads {
				dl.cats.2.get_or_insert("".into());
			}

			let tera = Tera::new("_templates/**/*")?;
			let mut context = Context::new();
			context.try_insert("app", &app)?;
			context.try_insert("genver", &env!("CARGO_PKG_VERSION"))?;
			context.try_insert("meta", &meta)?;
			context.try_insert("tag", &app.tag(&meta.version)?)?;
			let page = tera.render("release-page.html", &context)?;

			let dirpath = app.dir(&meta.version);
			create_dir_all(&dirpath).await?;

			let filepath = dirpath.join("index.html");
			let mut file = File::create(&filepath).await?;
			file.write_all(page.as_bytes()).await?;

			eprintln!("wrote {} bytes to {}", page.as_bytes().len(), filepath.display());
		}
	}

	return Ok(());
}
