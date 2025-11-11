use crate::models::track::Track;

pub struct PlayerService;

impl PlayerService {
	pub fn new() -> Self {
		Self
	}

	pub async fn play(&self, _track: &Track) -> Result<(), String> {
		Ok(())
	}

	pub async fn pause(&self) -> Result<(), String> {
		Ok(())
	}

	pub async fn stop(&self) -> Result<(), String> {
		Ok(())
	}
}


