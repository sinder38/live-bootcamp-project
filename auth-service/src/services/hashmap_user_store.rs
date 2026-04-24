use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Clone, Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.

        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let user = self.users.get(email);
        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
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

    #[test]
    fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        // Adding a new user should succeed
        assert_eq!(store.add_user(user.clone()), Ok(()));

        // Adding the same user again should fail with UserAlreadyExists
        assert_eq!(store.add_user(user), Err(UserStoreError::UserAlreadyExists));
    }

    #[test]
    fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        // Getting a non-existent user should fail with UserNotFound
        assert_eq!(
            store.get_user(&user.email),
            Err(UserStoreError::UserNotFound)
        );

        store.add_user(user.clone()).unwrap();

        // Getting an existing user should return the user
        assert_eq!(store.get_user(&user.email), Ok(user));
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = new_test_user();

        store.add_user(user.clone()).unwrap();

        // Correct credentials should succeed
        assert_eq!(store.validate_user(&user.email, &user.password), Ok(()));

        // Wrong password should fail with InvalidCredentials
        assert_eq!(
            store.validate_user(&user.email, "wrongpassword"),
            Err(UserStoreError::InvalidCredentials)
        );

        // Non-existent user should fail with UserNotFound
        assert_eq!(
            store.validate_user("nobody@example.com", "password123"),
            Err(UserStoreError::UserNotFound)
        );
    }
}
