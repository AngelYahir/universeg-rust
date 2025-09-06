use universeg_api::interface::rest::dto::auth::LoginDto;
use validator::Validate;

#[cfg(test)]
mod login_dto_validation_tests {
    use super::*;

    #[test]
    fn test_valid_login_dto() {
        let dto = LoginDto {
            email: "user@universeg.gg".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_valid_login_dto_with_minimal_password() {
        let dto = LoginDto {
            email: "test@example.com".to_string(),
            password: "Aa1@bcde".to_string(), // Exactly 8 chars with all requirements
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_valid_login_dto_with_all_special_chars() {
        let dto = LoginDto {
            email: "user@domain.co".to_string(),
            password: "Password1@$!%*?&".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    // Email validation tests
    #[test]
    fn test_invalid_email_no_at_symbol() {
        let dto = LoginDto {
            email: "userdomaincom".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_invalid_email_no_domain() {
        let dto = LoginDto {
            email: "user@".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_invalid_email_no_username() {
        let dto = LoginDto {
            email: "@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_invalid_email_multiple_at_symbols() {
        let dto = LoginDto {
            email: "user@@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_empty_email() {
        let dto = LoginDto {
            email: "".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    // Password validation tests
    #[test]
    fn test_password_too_short() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Aa1@".to_string(), // Only 4 chars
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_missing_uppercase() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "lowercase123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_missing_lowercase() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "UPPERCASE123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_missing_digit() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "ValidPassword!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_missing_special_char() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "ValidPassword123".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_with_invalid_special_chars() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "ValidPassword123#".to_string(), // # is not allowed
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_with_spaces() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Valid Password123!".to_string(), // Space not allowed
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_password_with_unicode_chars() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "ValidPassword123ñ!".to_string(), // ñ not allowed
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_empty_password() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    // Combined validation tests
    #[test]
    fn test_both_email_and_password_invalid() {
        let dto = LoginDto {
            email: "invalid-email".to_string(),
            password: "weak".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
        assert!(errors.field_errors().contains_key("password"));
    }

    // Edge cases
    #[test]
    fn test_password_with_only_special_chars_allowed() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1@".to_string(), // Only @ as special char
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_dollar_sign() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1$".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_exclamation() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_percent() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1%".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_asterisk() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1*".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_question_mark() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1?".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_password_with_ampersand() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "Password1&".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_very_long_valid_password() {
        let dto = LoginDto {
            email: "user@domain.com".to_string(),
            password: "VeryLongValidPassword123!WithAllRequiredCharacters".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_international_email_domain() {
        let dto = LoginDto {
            email: "user@internationaldomain.co.uk".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }
}
