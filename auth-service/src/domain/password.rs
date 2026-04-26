const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, String> {
        if validate_password(&password) {
            Ok(Self(password))
        } else {
            Err(format!(
                "Password must be at least {} characters long",
                MIN_PASSWORD_LENGTH
            ))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= MIN_PASSWORD_LENGTH
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    /// Generates strings of at least MIN_PASSWORD_LENGTH characters
    #[derive(Debug, Clone)]
    struct ValidPassword(String);

    impl Arbitrary for ValidPassword {
        fn arbitrary(g: &mut Gen) -> Self {
            let base = String::arbitrary(g);
            let padding = "a".repeat(MIN_PASSWORD_LENGTH);
            ValidPassword(format!("{}{}", base, padding))
        }
    }

    /// Generates ASCII strings shorter than MIN_PASSWORD_LENGTH bytes
    #[derive(Debug, Clone)]
    struct TooShortPassword(String);

    impl Arbitrary for TooShortPassword {
        fn arbitrary(g: &mut Gen) -> Self {
            let len = usize::arbitrary(g) % MIN_PASSWORD_LENGTH;
            TooShortPassword("a".repeat(len))
        }
    }

    #[test]
    fn valid_password_is_parsed_successfully() {
        assert!(Password::parse("password123".to_string()).is_ok());
    }

    #[test]
    fn password_too_short_is_rejected() {
        assert!(Password::parse("short".to_string()).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        assert!(Password::parse("".to_string()).is_err());
    }

    #[test]
    fn exactly_min_length_is_accepted() {
        let password = "a".repeat(MIN_PASSWORD_LENGTH);
        assert!(Password::parse(password).is_ok());
    }

    #[test]
    fn one_below_min_length_is_rejected() {
        let password = "a".repeat(MIN_PASSWORD_LENGTH - 1);
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn parsed_password_preserves_original_value() {
        let password = "securepass".to_string();
        let parsed = Password::parse(password.clone()).unwrap();
        assert_eq!(parsed.as_ref(), password);
    }

    /// Property: any string of at least MIN_PASSWORD_LENGTH chars should be accepted
    #[quickcheck]
    fn prop_valid_passwords_are_accepted(password: ValidPassword) -> bool {
        Password::parse(password.0).is_ok()
    }

    /// Property: any string shorter than MIN_PASSWORD_LENGTH must be rejected
    #[quickcheck]
    fn prop_too_short_is_always_rejected(input: TooShortPassword) -> bool {
        Password::parse(input.0).is_err()
    }

    /// Property: parsing preserves the original value
    #[quickcheck]
    fn prop_parsed_password_preserves_value(password: ValidPassword) -> bool {
        match Password::parse(password.0.clone()) {
            Ok(parsed) => parsed.as_ref() == password.0,
            Err(_) => false,
        }
    }
}
