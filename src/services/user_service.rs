use crate::error::AppError;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::repositories::user_repo::UserRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, payload: CreateUser) -> Result<User, AppError> {
        self.repo.create_user(payload).await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, AppError> {
        self.repo.get_user(id).await
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        self.repo.get_users().await
    }

    pub async fn update_user(&self, id: Uuid, payload: UpdateUser) -> Result<User, AppError> {
        self.repo.update_user(id, payload).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_user(id).await
    }
}
