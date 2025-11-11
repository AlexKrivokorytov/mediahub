pub struct AuthService;

impl AuthService {
	pub fn new() -> Self {
		Self
	}

	pub async fn login_with_token(&self, _provider: &str, _token: &str) -> Result<(), String> {
		Ok(())
	}
}


