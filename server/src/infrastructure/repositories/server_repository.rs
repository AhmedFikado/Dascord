use crate::domain::entities::Server;
use crate::domain::value_objects::ServerRole;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[async_trait]
pub trait ServerRepository: Send + Sync + Clone {
    async fn create(&self, server: Server) -> AppResult<Server>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Server>>;
    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<Server>>;
    async fn update(&self, server: Server) -> AppResult<Server>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    async fn add_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn remove_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn is_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<bool>;
    async fn get_members(&self, server_id: Uuid) -> AppResult<Vec<(Uuid, ServerRole)>>;
    async fn get_member_role(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Option<ServerRole>>;
    async fn update_member_role(&self, server_id: Uuid, user_id: Uuid, role: ServerRole) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PostgresServerRepository {
    pool: PgPool,
}

impl PostgresServerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerRepository for PostgresServerRepository {
    async fn create(&self, server: Server) -> AppResult<Server> {
        sqlx::query(
            "INSERT INTO servers (id, name, owner_id, invitation_code, created_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(server.id)
        .bind(&server.name)
        .bind(server.owner_id)
        .bind(&server.invitation_code)
        .bind(server.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        sqlx::query(
            "INSERT INTO server_members (server_id, user_id, role) VALUES ($1, $2, $3::role_type)"
        )
        .bind(server.id)
        .bind(server.owner_id)
        .bind("OWNER")
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(server)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Server>> {
        let result = sqlx::query_as::<_, Server>(
            "SELECT id, name, owner_id, invitation_code, created_at FROM servers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<Server>> {
        let servers = sqlx::query_as::<_, Server>(
            "SELECT s.id, s.name, s.owner_id, s.invitation_code, s.created_at 
             FROM servers s 
             INNER JOIN server_members sm ON s.id = sm.server_id 
             WHERE sm.user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(servers)
    }

    async fn update(&self, server: Server) -> AppResult<Server> {
        sqlx::query(
            "UPDATE servers SET name = $1 WHERE id = $2"
        )
        .bind(&server.name)
        .bind(server.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(server)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM servers WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn add_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO server_members (server_id, user_id, role) VALUES ($1, $2, $3::role_type)"
        )
        .bind(server_id)
        .bind(user_id)
        .bind("MEMBER")
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn remove_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM server_members WHERE server_id = $1 AND user_id = $2"
        )
        .bind(server_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn is_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2)"
        )
        .bind(server_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.get::<bool, _>(0))
    }

    async fn get_members(&self, server_id: Uuid) -> AppResult<Vec<(Uuid, ServerRole)>> {
        let rows = sqlx::query(
            "SELECT user_id, role FROM server_members WHERE server_id = $1"
        )
        .bind(server_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let members = rows
            .into_iter()
            .map(|row| {
                let user_id: Uuid = row.get("user_id");
                let role_str: String = row.get("role");
                let role = match role_str.as_str() {
                    "OWNER" => ServerRole::Owner,
                    "ADMIN" => ServerRole::Admin,
                    _ => ServerRole::Member,
                };
                (user_id, role)
            })
            .collect();

        Ok(members)
    }

    async fn get_member_role(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Option<ServerRole>> {
        let result = sqlx::query(
            "SELECT role FROM server_members WHERE server_id = $1 AND user_id = $2"
        )
        .bind(server_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.map(|row| {
            let role_str: String = row.get("role");
            match role_str.as_str() {
                "OWNER" => ServerRole::Owner,
                "ADMIN" => ServerRole::Admin,
                _ => ServerRole::Member,
            }
        }))
    }

    async fn update_member_role(&self, server_id: Uuid, user_id: Uuid, role: ServerRole) -> AppResult<()> {
        let role_str = match role {
            ServerRole::Owner => "OWNER",
            ServerRole::Admin => "ADMIN",
            ServerRole::Member => "MEMBER",
        };

        sqlx::query(
            "UPDATE server_members SET role = $1::role_type WHERE server_id = $2 AND user_id = $3"
        )
        .bind(role_str)
        .bind(server_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
