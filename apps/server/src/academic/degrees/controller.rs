use crate::academic::*;
use crate::auth::SessionCheck;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/degrees")]
#[interceptor(SessionCheck)]
pub struct DegreesController {
	degrees: Arc<DegreesService>,
}

impl DegreesController {
	#[get("/academic/{id}")]
	pub async fn get_degrees(&self, req: Request) -> WebResult<Vec<Degree>> {
		let academic_id = req.param::<AcademicId>("id")?;
		let degrees = self.degrees.find(&academic_id).await?;

		Ok(degrees)
	}

	#[post("/")]
	pub async fn create_degree(&self, req: Request) -> WebResult<Degree> {
		let input = req.body_validator::<CreateDegreeDto>()?;
		let degree = self.degrees.create(input).await?;

		Ok(degree)
	}

	#[patch("/{id}")]
	pub async fn update_degree(&self, req: Request) -> WebResult<Degree> {
		let degree_id = req.param::<DegreeId>("id")?;
		let input = req.body_validator::<UpdateDegreeDto>()?;

		let degree = self.degrees.update(&degree_id, input).await?;

		Ok(degree)
	}
}
