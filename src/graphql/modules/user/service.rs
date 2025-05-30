use crate::graphql::modules::user::model::User;
use crate::graphql::modules::user::repository::UserRepository;
use mongodb::Database;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(db: &Database) -> Self {
        let repository = UserRepository::new(db.collection("users"));
        Self { repository }
    }
    
    pub async fn find_all(&self) -> Vec<User> {
        self.repository.find_all().await
    }
    
    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        self.repository.find_by_id(id).await
    }
    
    pub async fn create(&self, name: String, email: String, age: Option<i32>) -> Result<User, String> {
        let user = User::new(name, email, age);
        self.repository.create(user).await
    }
}
