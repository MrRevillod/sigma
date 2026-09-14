use crate::auth::*;
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersService {
	users: Arc<UsersRepository>,
	hasher: Arc<Hasher>,
}

impl UsersService {
	pub async fn find(&self, query: GetUsersQuery) -> AppResult<Vec<UserView>> {
		let filter = UserFilter {
			role: query.role,
			search: query.search,
		};

		self.users.list(filter).await
	}

	pub async fn find_by_id(&self, id: &UserId) -> AppResult<UserView> {
		let Some(user) = self.users.find_by_id(id).await? else {
			return Err(AuthError::UserNotFound)?;
		};

		Ok(UserView::from(user))
	}

	pub async fn create(&self, dto: CreateUserDto) -> AppResult<UserView> {
		if self.users.find_by_email(&dto.email).await?.is_some() {
			Err(AuthError::EmailAlreadyExists)?;
		}

		let user = User::builder()
			.name(dto.name)
			.email(dto.email)
			.role(dto.role)
			.password_hash(self.hasher.hash(&dto.password)?)
			.build();

		self.users.save(&user).await?;

		Ok(UserView::from(user))
	}

	pub async fn update(&self, id: &UserId, dto: UpdateUserDto) -> AppResult<UserView> {
		let Some(mut user) = self.users.find_by_id(id).await? else {
			return Err(AuthError::UserNotFound)?;
		};

		if let Some(ref email) = dto.email
			&& email != &user.email
			&& self.users.find_by_email(email).await?.is_some()
		{
			Err(AuthError::EmailAlreadyExists)?;
		}

		if let Some(name) = dto.name {
			user.name = name;
		}

		if let Some(email) = dto.email {
			user.email = email;
		}

		if let Some(role) = dto.role {
			user.role = role;
		}

		if let Some(password) = dto.password {
			user.password_hash = self.hasher.hash(&password)?;
		}

		self.users.save(&user).await?;

		Ok(UserView::from(user))
	}

	pub async fn delete(&self, id: &UserId) -> AppResult<()> {
		if self.users.find_by_id(id).await?.is_none() {
			Err(AuthError::UserNotFound)?;
		}

		self.users.delete(id).await
	}
}
