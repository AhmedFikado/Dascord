use uuid::Uuid;
use crate::utils::error::{AppError, AppResult};
use crate::domain::entities::channel::PrivateChannel;
use crate::infrastructure::repositories::channel::PrivateChannelRepository;
use crate::infrastructure::repositories::UserRepository;

pub struct PrivateChannelService<PCR: PrivateChannelRepository, UR: UserRepository> {
    private_channel_repository: PCR,
    user_repository: UR,
}

impl<PCR: PrivateChannelRepository, UR: UserRepository> PrivateChannelService<PCR, UR> {
    pub fn new(private_channel_repository: PCR, user_repository: UR) -> Self {
        Self {
            private_channel_repository,
            user_repository,
        }
    }

    pub async fn create_private_channel(&self, user1: Uuid, user2: Uuid) -> AppResult<PrivateChannel> {
        if user1 == user2 {
            return Err(AppError::BadRequest("Cannot create a private channel with yourself".to_string()));
        }

        // Check if both users exist
        if self.user_repository.find_by_id(user1).await?.is_none() {
            return Err(AppError::NotFound("User 1 not found".to_string()));
        }

        if self.user_repository.find_by_id(user2).await?.is_none() {
            return Err(AppError::NotFound("User 2 not found".to_string()));
        }

        // Check if channel already exists
        if let Ok(Some(_)) = self.private_channel_repository.get_by_users(user1, user2).await {
            return Err(AppError::BadRequest("Private channel already exists".to_string()));
        }

        self.private_channel_repository.create(user1, user2).await
    }

    pub async fn get_private_channel(&self, id: Uuid) -> AppResult<Option<PrivateChannel>> {
        self.private_channel_repository.get_by_id(id).await
    }

    pub async fn get_user_private_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>> {
        self.private_channel_repository.get_user_channels(user_id).await
    }

    pub async fn get_user_private_channels_with_recipient(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<(PrivateChannel, Option<crate::domain::entities::user::User>)>> {
        let channels = self.private_channel_repository.get_user_channels(user_id).await?;
        let mut result = Vec::new();
        for channel in channels {
            let recipient_id = if channel.user1 == user_id { channel.user2 } else { channel.user1 };
            let recipient = self.user_repository.find_by_id(recipient_id).await?;
            result.push((channel, recipient));
        }
        Ok(result)
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::User;
    use chrono::Utc;

    // Mock implementations for testing
    #[derive(Clone)]
    struct MockPrivateChannelRepository {
        channels: Vec<PrivateChannel>,
    }

    #[async_trait::async_trait]
    impl PrivateChannelRepository for MockPrivateChannelRepository {
        async fn create(&self, user1: Uuid, user2: Uuid) -> AppResult<PrivateChannel> {
            let channel = PrivateChannel {
                id: Uuid::new_v4(),
                user1,
                user2,
                created_at: Utc::now(),
            };
            Ok(channel)
        }

        async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PrivateChannel>> {
            Ok(self.channels.iter().find(|c| c.id == id).cloned())
        }

        async fn get_by_users(&self, user1: Uuid, user2: Uuid) -> AppResult<Option<PrivateChannel>> {
            Ok(self.channels.iter().find(|c| (c.user1 == user1 && c.user2 == user2) || (c.user1 == user2 && c.user2 == user1)).cloned())
        }

        async fn get_user_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>> {
            Ok(self.channels.iter().filter(|c| c.user1 == user_id || c.user2 == user_id).cloned().collect())
        }
    }

    #[derive(Clone)]
    struct MockUserRepository {
        users: Vec<Uuid>,
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: User) -> AppResult<User> {
            Ok(user)
        }

        async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
            Ok(self.users.iter().find(|u| **u == id).map(|_| {
                User {
                    id,
                    username: "test_user".to_string(),
                    email: "test@example.com".to_string(),
                    language: "fr".to_string(),
                    password_hash: "hash".to_string(),
                    status: "ONLINE".to_string(),
                    created_at: Utc::now(),
                }
            }))
        }

        async fn find_by_email(&self, _email: &str) -> AppResult<Option<User>> {
            Ok(None)
        }

        async fn find_by_username(&self, _username: &str) -> AppResult<Option<User>> {
            Ok(None)
        }

        async fn update_status(&self, _id: Uuid, _status: &str) -> AppResult<()> {
            Ok(())
        }

        async fn update_user(&self, user: User) -> AppResult<Option<User>> {
            Ok(Some(user))
        }
    }

    #[tokio::test]
    async fn test_create_private_channel_success() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let repo = MockPrivateChannelRepository { channels: vec![] };
        let user_repo = MockUserRepository { users: vec![user1, user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.create_private_channel(user1, user2).await;
        assert!(result.is_ok());
        let channel = result.unwrap();
        assert_eq!(channel.user1, user1);
        assert_eq!(channel.user2, user2);
    }

    #[tokio::test]
    async fn test_create_private_channel_same_user() {
        let user1 = Uuid::new_v4();
        
        let repo = MockPrivateChannelRepository { channels: vec![] };
        let user_repo = MockUserRepository { users: vec![user1] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.create_private_channel(user1, user1).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Cannot create a private channel with yourself");
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_create_private_channel_user1_not_found() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let repo = MockPrivateChannelRepository { channels: vec![] };
        let user_repo = MockUserRepository { users: vec![user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.create_private_channel(user1, user2).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => {
                assert_eq!(msg, "User 1 not found");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_create_private_channel_user2_not_found() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let repo = MockPrivateChannelRepository { channels: vec![] };
        let user_repo = MockUserRepository { users: vec![user1] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.create_private_channel(user1, user2).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => {
                assert_eq!(msg, "User 2 not found");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_create_private_channel_already_exists() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let existing_channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1,
            user2,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository { channels: vec![existing_channel] };
        let user_repo = MockUserRepository { users: vec![user1, user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.create_private_channel(user1, user2).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Private channel already exists");
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_create_private_channel_already_exists_reversed_order() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let existing_channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1,
            user2,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository { channels: vec![existing_channel] };
        let user_repo = MockUserRepository { users: vec![user1, user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        // Try creating with reversed order - should still detect existing channel
        let result = service.create_private_channel(user2, user1).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Private channel already exists");
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_get_private_channel_success() {
        let channel_id = Uuid::new_v4();
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let channel = PrivateChannel {
            id: channel_id,
            user1,
            user2,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository { channels: vec![channel.clone()] };
        let user_repo = MockUserRepository { users: vec![user1, user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.get_private_channel(channel_id).await;
        assert!(result.is_ok());
        let found_channel = result.unwrap();
        assert!(found_channel.is_some());
        assert_eq!(found_channel.unwrap().id, channel_id);
    }

    #[tokio::test]
    async fn test_get_private_channel_not_found() {
        let channel_id = Uuid::new_v4();
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        let repo = MockPrivateChannelRepository { channels: vec![] };
        let user_repo = MockUserRepository { users: vec![user1, user2] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.get_private_channel(channel_id).await;
        assert!(result.is_ok());
        let found_channel = result.unwrap();
        assert!(found_channel.is_none());
    }

    #[tokio::test]
    async fn test_get_user_private_channels_success() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let user3 = Uuid::new_v4();
        
        let channel1 = PrivateChannel {
            id: Uuid::new_v4(),
            user1,
            user2,
            created_at: Utc::now(),
        };
        
        let channel2 = PrivateChannel {
            id: Uuid::new_v4(),
            user1,
            user2: user3,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository {
            channels: vec![channel1.clone(), channel2.clone()],
        };
        let user_repo = MockUserRepository { users: vec![user1, user2, user3] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.get_user_private_channels(user1).await;
        assert!(result.is_ok());
        let channels = result.unwrap();
        assert_eq!(channels.len(), 2);
    }

    #[tokio::test]
    async fn test_get_user_private_channels_empty() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let user3 = Uuid::new_v4();
        
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: user2,
            user2: user3,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository { channels: vec![channel] };
        let user_repo = MockUserRepository { users: vec![user1, user2, user3] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.get_user_private_channels(user1).await;
        assert!(result.is_ok());
        let channels = result.unwrap();
        assert_eq!(channels.len(), 0);
    }

    #[tokio::test]
    async fn test_get_user_private_channels_filters_correctly() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let user3 = Uuid::new_v4();
        let user4 = Uuid::new_v4();
        
        let channel1 = PrivateChannel {
            id: Uuid::new_v4(),
            user1,
            user2,
            created_at: Utc::now(),
        };
        
        let channel2 = PrivateChannel {
            id: Uuid::new_v4(),
            user1: user3,
            user2: user4,
            created_at: Utc::now(),
        };
        
        let channel3 = PrivateChannel {
            id: Uuid::new_v4(),
            user1: user2,
            user2,
            created_at: Utc::now(),
        };
        
        let repo = MockPrivateChannelRepository {
            channels: vec![channel1.clone(), channel2.clone(), channel3.clone()],
        };
        let user_repo = MockUserRepository { users: vec![user1, user2, user3, user4] };
        
        let service = PrivateChannelService::new(repo, user_repo);
        
        let result = service.get_user_private_channels(user1).await;
        assert!(result.is_ok());
        let channels = result.unwrap();
        // user1 should only be in channel1
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0].id, channel1.id);
    }
}
