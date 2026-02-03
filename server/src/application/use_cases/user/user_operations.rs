use crate::application::dto::user_dto::UserDto;
use crate::infrastructure::services::UserService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct GetUserInfoUseCase {
    user_service: UserService,
}

impl GetUserInfoUseCase {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn execute(&self, user_id: Uuid) -> AppResult<UserDto> {
        let user = self.user_service.find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        Ok(UserDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            status: user.status,
        })
    }
}
