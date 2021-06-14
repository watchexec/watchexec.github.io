use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use async_std::{fs::File, prelude::*};
use color_eyre::{eyre::eyre, Result};
use globset::Glob;
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Deserializer};
use url::Url;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
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
	hs.into_iter()
		.map(|(k, v)| Glob::new(&k).map(|g| (g, v)))
		.collect::<Result<_, _>>()
		.map_err(|err| serde::de::Error::custom(err.to_string()))
}

fn exts_globs<'de, D>(deserializer: D) -> Result<HashMap<String, Glob>, D::Error>
where
	D: Deserializer<'de>,
{
	let hs = HashMap::<String, String>::deserialize(deserializer)?;
	hs.into_iter()
		.map(|(k, v)| Glob::new(&v).map(|g| (k, g)))
		.collect::<Result<_, _>>()
		.map_err(|err| serde::de::Error::custom(err.to_string()))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Maintainer {
	pub username: String,
	pub name: String,
	pub homepage: Option<Url>,
	pub key_url: Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppName {
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
pub enum NotesSource {
	#[serde(rename = "gh-release")]
	GithubRelease,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SumAlgo {
	#[serde(rename = "BLAKE3")]
	Blake3,

	#[serde(rename = "SHA512")]
	Sha512,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tri<T> {
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

impl<T> Tri<T> {
	/// Returns `other` iff `self` is `NotPresent`
	pub fn or(self, other: Self) -> Self {
		match self {
			Tri::NotPresent => other,
			tri => tri,
		}
	}

	pub fn override_with(&mut self, other: Self) {
		match other {
			Tri::NotPresent => {},
			o => {
				*self = o;
			}
		}
	}
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct App {
	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub repo: String,

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
pub struct Prior {
	pub before: Version,

	#[serde(flatten)]
	pub app: App,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cat {
	#[serde(default)]
	pub arch: Option<String>,

	#[serde(default)]
	pub os: Option<String>,

	#[serde(default)]
	pub variant: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChecksumDetail {
	pub filename: String,
	pub tool: String,
	pub url: Url,
}

impl Config {
	pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
		let mut file = File::open(path.as_ref()).await?;
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).await?;
		let mut cfg: Config = serde_yaml::from_slice(&bytes)?;

		for (name, app) in &mut cfg.apps {
			app.priors.sort_by_key(|p| p.before.clone());

			if let AppName::Named(name) = name {
				if app.repo.is_empty() {
					return Err(eyre!("'repo' is required, but not found on app '{}'", name));
				}

				for prior in &app.priors {
					if !prior.app.priors.is_empty() {
						return Err(eyre!("{}'s prior (before={}) has a 'priors' key, this makes no sense", name, prior.before));
					}
				}
			}
		}

		Ok(cfg)
	}

	pub fn app(&self, name: &str, version: &Version) -> Result<App> {
		let mut app = self
			.apps
			.get(&AppName::Named(name.to_string()))
			.ok_or_else(|| eyre!("no such app defined: {}", name))?
			.clone();

		let defaults = self
			.apps
			.get(&AppName::Defaults)
			.cloned()
			.unwrap_or_default();

		if app.name.is_empty() {
			app.name = name.to_string(); // TODO: ucfirst
		}

		app.tag_format = Some(
			app.tag_format
				.or(defaults.tag_format)
				.ok_or_else(|| eyre!("app '{}' is missing its 'tag_format'", name))?,
		);

		app.basename_format = app.basename_format.or(defaults.basename_format);
		app.notes = app.notes.or(defaults.notes);
		app.checksums = app.checksums.or(defaults.checksums);
		app.packings = app.packings.or(defaults.packings);
		app.key_path = app.key_path.or(defaults.key_path);

		// priors here are in ASC order, unlike in the config, where they (most
		// likely) are in DESC order (most recent at the top)
		let priors = match app.priors.binary_search_by_key(version, |p| p.before.clone()) {
			Ok(pidx) => if pidx == app.priors.len() - 1 {
				// matching as the version the last prior is before,
				// i.e. current
				&[]
			} else {
				// matching as the version a prior is before,
				// i.e. we want from the next one up
				&app.priors[(pidx + 1)..]
			},
			Err(pidx) => if pidx == app.priors.len() {
				// matching as after the version the last prior is before,
				// i.e. current
				&[]
			} else {
				// matching as after the version a prior is before,
				// i.e. we want from that one up
				&app.priors[pidx..]
			},
		};

		// we actually want to apply the priors' overrides in DESC order
		for Prior { app: pa, .. } in priors.iter().rev() {
			let pa = pa.clone();

			if let Some(tf) = pa.tag_format {
				app.tag_format.replace(tf);
			}

			if let Some(bf) = pa.basename_format {
				app.basename_format.replace(bf);
			}

			if let Some(kp) = pa.key_path {
				app.key_path.replace(kp);
			}

			app.notes.override_with(pa.notes);
			app.checksums.override_with(pa.checksums);
			app.packings.override_with(pa.packings);
		}

		Ok(app)
	}
}
