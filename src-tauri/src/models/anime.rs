use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
	pub id: String,
	pub title: String,
	pub episodes: u32,
}


