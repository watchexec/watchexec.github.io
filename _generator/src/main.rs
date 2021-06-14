use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use async_std::{fs::File, prelude::*};
use color_eyre::Result;
use globset::Glob;
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Deserializer};
use structopt::StructOpt;
use url::Url;

#[derive(Clone, Debug, StructOpt)]
enum Mode {
	Lint {
		#[structopt(short, long = "config")]
		config_file: PathBuf,

		#[structopt(long)]
		print: bool,
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
			return Ok(());
		}
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
	pub maintainers: Vec<Maintainer>,
	pub apps: HashMap<AppName, App>,
	#[serde(deserialize_with = "triple_globs")]
	pub triples: IndexMap<Glob, Cat>,
	#[serde(deserialize_with = "exts_globs")]
	pub extensions: HashMap<String, Glob>,
	pub checksums: HashMap<SumAlgo, ChecksumDetail>,
}

fn triple_globs<'de, D>(deserializer: D) -> Result<IndexMap<Glob, Cat>, D::Error>
where
	D: Deserializer<'de>,
{
	let hs = IndexMap::<String, Cat>::deserialize(deserializer)?;
	hs
		.into_iter()
		.map(|(k, v)| Glob::new(&k).map(|g| (g, v)))
		.collect::<Result<_, _>>()
		.map_err(|err| serde::de::Error::custom(err.to_string()))
}

fn exts_globs<'de, D>(deserializer: D) -> Result<HashMap<String, Glob>, D::Error>
where
	D: Deserializer<'de>,
{
	let hs = HashMap::<String, String>::deserialize(deserializer)?;
	hs
		.into_iter()
		.map(|(k, v)| Glob::new(&v).map(|g| (k, g)))
		.collect::<Result<_, _>>()
		.map_err(|err| serde::de::Error::custom(err.to_string()))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Maintainer {
	pub username: String,
	pub name: String,
	pub homepage: Option<Url>,
	pub key_url: Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppName {
	Defaults,
	Named(String),
}

impl<'de> Deserialize<'de> for AppName {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let name = String::deserialize(deserializer)?;
		if name == "defaults" {
			Ok(Self::Defaults)
		} else {
			Ok(Self::Named(name))
		}
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NotesSource {
	#[serde(rename = "gh-release")]
	GithubRelease,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SumAlgo {
	#[serde(rename = "BLAKE3")]
	Blake3,

	#[serde(rename = "SHA512")]
	Sha512,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tri<T> {
	NotPresent,
	Disabled,
	Set(T),
}

impl<T> Default for Tri<T> {
	fn default() -> Self {
		Self::NotPresent
	}
}

impl<'de, T> Deserialize<'de> for Tri<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum BoolOr<T> {
			Bool(bool),
			Some(T),
		}

		type NoneBoolOr<T> = Option<BoolOr<T>>;

		match NoneBoolOr::<T>::deserialize(deserializer)? {
			None => Ok(Self::NotPresent),
			Some(BoolOr::Bool(true)) => Err(serde::de::Error::custom("cannot be true")),
			Some(BoolOr::Bool(false)) => Ok(Self::Disabled),
			Some(BoolOr::Some(t)) => Ok(Self::Set(t)),
		}
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct App {
	#[serde(default)]
	pub name: Option<String>,

	#[serde(default)]
	pub repo: Option<String>,

	#[serde(default)]
	pub key_path: Option<PathBuf>,

	#[serde(default)]
	pub tag_format: Option<String>,

	#[serde(default)]
	pub basename_format: Option<String>,

	#[serde(default)]
	pub notes: Tri<NotesSource>,

	#[serde(default)]
	pub checksums: Tri<Vec<SumAlgo>>,

	#[serde(default, deserialize_with = "packing_globs")]
	pub packings: Tri<IndexMap<Glob, Vec<String>>>,

	#[serde(default)]
	pub priors: Vec<Prior>,
}

fn packing_globs<'de, D>(deserializer: D) -> Result<Tri<IndexMap<Glob, Vec<String>>>, D::Error>
where
	D: Deserializer<'de>,
{
	match Tri::<IndexMap<String, Vec<String>>>::deserialize(deserializer)? {
		Tri::Set(ig) => ig
			.into_iter()
			.map(|(k, v)| Glob::new(&k).map(|g| (g, v)))
			.collect::<Result<_, _>>()
			.map(Tri::Set)
			.map_err(|err| serde::de::Error::custom(err.to_string())),
		Tri::NotPresent => Ok(Tri::NotPresent),
		Tri::Disabled => Ok(Tri::Disabled),
	}
}

#[derive(Clone, Debug, Deserialize)]
struct Prior {
	pub before: Version,

	#[serde(flatten)]
	pub app: App,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cat {
	#[serde(default)]
	pub arch: Option<String>,

	#[serde(default)]
	pub os: Option<String>,

	#[serde(default)]
	pub variant: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChecksumDetail {
	pub filename: String,
	pub tool: String,
	pub url: Url,
}

impl Config {
	pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
		let mut file = File::open(path.as_ref()).await?;
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).await?;
		Ok(serde_yaml::from_slice(&bytes)?)
	}
}
