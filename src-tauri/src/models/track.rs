use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
	pub id: String,
	pub title: String,
	pub artist: String,
	pub duration_ms: u64,
}


