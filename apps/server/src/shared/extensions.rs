use crate::auth::SessionClaims;

use sword::web::Request;

pub trait RequestExt {
	fn claims(&self) -> Option<&SessionClaims>;
}

impl RequestExt for Request {
	fn claims(&self) -> Option<&SessionClaims> {
		self.extensions.get::<SessionClaims>()
	}
}
