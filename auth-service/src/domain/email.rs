pub struct Email(String);

impl Email {
    fn parse(email: String) -> Result<Self, String> {
        if validate_email(&email) {
            Ok(Email(email))
        } else {
            Err("Invalid email".to_string())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn validate_email(s: &str) -> bool {
    if s.contains('@') {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    /// Generates valid-looking emails using `fake`
    #[derive(Debug, Clone)]
    struct ValidEmail(String);

    impl Arbitrary for ValidEmail {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed = u64::arbitrary(g);
            let mut rng = SmallRng::seed_from_u64(seed);
            ValidEmail(SafeEmail().fake_with_rng(&mut rng))
        }
    }

    /// Generates strings that don't contain '@' character
    #[derive(Debug, Clone)]
    struct NoAtCharacter(String);

    impl Arbitrary for NoAtCharacter {
        fn arbitrary(g: &mut Gen) -> Self {
            let s = String::arbitrary(g).chars().filter(|&c| c != '@').collect();
            NoAtCharacter(s)
        }
    }

    #[test]
    fn valid_email_is_parsed_successfully() {
        assert!(Email::parse("user@example.com".to_string()).is_ok());
    }

    #[test]
    fn email_without_at_sign_is_rejected() {
        assert!(Email::parse("invalidemail.com".to_string()).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        assert!(Email::parse("".to_string()).is_err());
    }

    #[test]
    fn parsed_email_preserves_original_value() {
        let email = "user@example.com".to_string();
        let parsed = Email::parse(email.clone()).unwrap();
        assert_eq!(parsed.as_ref(), email);
    }

    /// Property: any valid email should be accepted
    #[quickcheck]
    fn prop_valid_emails_are_accepted(email: ValidEmail) -> bool {
        Email::parse(email.0).is_ok()
    }

    /// Property: any string without '@' must be rejected
    #[quickcheck]
    fn prop_no_at_sign_is_always_rejected(input: NoAtCharacter) -> bool {
        Email::parse(input.0).is_err()
    }

    /// Property: parsing is idempotent
    #[quickcheck]
    fn prop_parsed_email_preserves_value(email: ValidEmail) -> bool {
        match Email::parse(email.0.clone()) {
            Ok(parsed) => parsed.as_ref() == email.0,
            Err(_) => false,
        }
    }
}
