use universeg_api::interface::rest::dto::auth::RegisterDto;
use validator::Validate;

#[cfg(test)]
mod register_dto_validation_tests {
    use super::*;

    #[test]
    fn test_valid_register_dto() {
        let dto = RegisterDto {
            username: "validuser".to_string(),
            email: "user@universeg.gg".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_valid_register_dto_with_minimal_requirements() {
        let dto = RegisterDto {
            username: "abc".to_string(),      // Minimum 3 characters
            email: "a@b.co".to_string(),      // Minimal valid email
            password: "Aa1@bcde".to_string(), // Minimum 8 chars with all requirements
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_valid_register_dto_with_max_username_length() {
        let dto = RegisterDto {
            username: "sixteencharacter".to_string(), // 16 characters
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    // Username validation tests
    #[test]
    fn test_username_too_short() {
        let dto = RegisterDto {
            username: "ab".to_string(), // Only 2 chars
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_username_too_long() {
        let dto = RegisterDto {
            username: "seventeencharacters".to_string(), // 17 characters
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_username_with_invalid_characters() {
        let dto = RegisterDto {
            username: "user-name".to_string(), // Contains hyphen
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_username_with_special_characters() {
        let dto = RegisterDto {
            username: "user@name".to_string(), // Contains @
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_username_with_spaces() {
        let dto = RegisterDto {
            username: "user name".to_string(), // Contains space
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_empty_username() {
        let dto = RegisterDto {
            username: "".to_string(),
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_username_with_underscore() {
        // Underscore should be allowed
        let dto = RegisterDto {
            username: "user_name".to_string(),
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_username_with_numbers() {
        // Numbers should be allowed
        let dto = RegisterDto {
            username: "user123".to_string(),
            email: "test@example.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    // Email validation tests
    #[test]
    fn test_invalid_email_no_at_symbol() {
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
            email: "@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_empty_email() {
        let dto = RegisterDto {
            username: "validuser".to_string(),
            email: "".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    // Password validation tests (same as login tests, as they share the same validation)
    #[test]
    fn test_password_too_short() {
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
        let dto = RegisterDto {
            username: "validuser".to_string(),
            email: "user@domain.com".to_string(),
            password: "ValidPassword123#".to_string(), // # is not allowed
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_empty_password() {
        let dto = RegisterDto {
            username: "validuser".to_string(),
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
    fn test_all_fields_invalid() {
        let dto = RegisterDto {
            username: "a".to_string(),          // Too short
            email: "invalid-email".to_string(), // Invalid email
            password: "weak".to_string(),       // Invalid password
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
        assert!(errors.field_errors().contains_key("email"));
        assert!(errors.field_errors().contains_key("password"));
    }

    // Edge cases
    #[test]
    fn test_username_exactly_three_chars() {
        let dto = RegisterDto {
            username: "abc".to_string(),
            email: "user@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_username_exactly_sixteen_chars() {
        let dto = RegisterDto {
            username: "abcdefghijklmnop".to_string(), // 16 chars
            email: "user@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_username_with_mixed_case() {
        let dto = RegisterDto {
            username: "UserName".to_string(),
            email: "user@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_username_with_complex_pattern() {
        let dto = RegisterDto {
            username: "User_123_Name".to_string(),
            email: "user@domain.com".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        let validation_result = dto.validate();
        assert!(validation_result.is_ok());
    }
}
