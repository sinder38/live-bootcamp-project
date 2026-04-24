use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Clone, Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_user() -> User {
        User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        }
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        assert_eq!(store.add_user(user.clone()).await, Ok(()));
        assert_eq!(
            store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        assert_eq!(
            store.get_user(&user.email).await,
            Err(UserStoreError::UserNotFound)
        );

        store.add_user(user.clone()).await.unwrap();

        assert_eq!(store.get_user(&user.email).await, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        store.add_user(user.clone()).await.unwrap();

        assert_eq!(store.validate_user(&user.email, &user.password).await, Ok(()));
        assert_eq!(
            store.validate_user(&user.email, "wrongpassword").await,
            Err(UserStoreError::InvalidCredentials)
        );
        assert_eq!(
            store.validate_user("nobody@example.com", "password123").await,
            Err(UserStoreError::UserNotFound)
        );
    }
}
