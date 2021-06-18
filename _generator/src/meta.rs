use std::path::Path;

use async_std::{fs::File, prelude::*};
use chrono::{DateTime, Utc};
use color_eyre::{eyre::eyre, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::{Cat, Config, Format, SumAlgo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
	pub generated: DateTime<Utc>,
	pub published: DateTime<Utc>,
	pub app: String,
	pub version: Version,
	pub notes: Option<String>,
	pub downloads: Vec<Download>,
	pub sums: Vec<SignedSum>,
}

impl Meta {
	pub fn new(app: String, version: Version, published: DateTime<Utc>, notes: Option<String>, downloads: Vec<Download>, sums: Vec<SignedSum>) -> Self {
		Self {
			generated: Utc::now(),
			published,
			app,
			version,
			notes,
			downloads,
			sums,
		}
	}

	pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
		let mut file = File::open(path.as_ref()).await?;
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).await?;
		Ok(serde_json::from_slice(&bytes)?)
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Download {
	pub url: Url,
	pub filename: String,
	pub size: usize,
	pub format: Format,
	pub sums: Vec<DownloadSum>,
	pub cats: (String, String, Option<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DownloadSum {
	pub algo: SumAlgo,
	pub sum: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedSum {
	pub algo: SumAlgo,
	pub url: Url,
	pub signs: Vec<Sign>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sign {
	pub sign_url: Url,
	pub key_url: Url,
	pub name: String,
}
