use universeg_api::interface::rest::dto::auth::validate_password;

#[cfg(test)]
mod password_validation_tests {
    use super::*;

    #[test]
    fn test_valid_password_minimal() {
        let password = "Aa1@bcde";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_all_special_chars() {
        let password = "Password1@$!%*?&";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_long() {
        let password = "ThisIsAVeryLongValidPassword123!";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_at_symbol() {
        let password = "ValidPass1@";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_dollar_sign() {
        let password = "ValidPass1$";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_exclamation() {
        let password = "ValidPass1!";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_percent() {
        let password = "ValidPass1%";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_asterisk() {
        let password = "ValidPass1*";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_question_mark() {
        let password = "ValidPass1?";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_with_ampersand() {
        let password = "ValidPass1&";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    // Length validation tests
    #[test]
    fn test_password_too_short() {
        let password = "Aa1@";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_length");
    }

    #[test]
    fn test_password_exactly_7_chars() {
        let password = "Aa1@bcd";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_length");
    }

    #[test]
    fn test_password_exactly_8_chars_valid() {
        let password = "Aa1@bcde";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_password() {
        let password = "";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_length");
    }

    // Character set validation tests
    #[test]
    fn test_password_with_invalid_special_chars() {
        let password = "ValidPass1#"; // # is not in allowed special chars
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_charset");
    }

    #[test]
    fn test_password_with_spaces() {
        let password = "Valid Pass1!"; // Space not allowed
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_charset");
    }

    #[test]
    fn test_password_with_unicode_chars() {
        let password = "ValidPassñ1!"; // ñ not allowed
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_charset");
    }

    #[test]
    fn test_password_with_brackets() {
        let password = "ValidPass1["; // [ not allowed
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_charset");
    }

    #[test]
    fn test_password_with_period() {
        let password = "ValidPass1."; // . not allowed
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_charset");
    }

    // Lowercase requirement tests
    #[test]
    fn test_password_missing_lowercase() {
        let password = "VALIDPASS1!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_lowercase");
    }

    #[test]
    fn test_password_with_only_numbers_and_specials() {
        let password = "12345678!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_lowercase");
    }

    // Uppercase requirement tests
    #[test]
    fn test_password_missing_uppercase() {
        let password = "validpass1!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_uppercase");
    }

    #[test]
    fn test_password_with_only_lowercase_and_specials() {
        let password = "validpass!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_uppercase");
    }

    // Digit requirement tests
    #[test]
    fn test_password_missing_digit() {
        let password = "ValidPass!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_digit");
    }

    #[test]
    fn test_password_with_only_letters_and_specials() {
        let password = "ValidPass!";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_digit");
    }

    // Special character requirement tests
    #[test]
    fn test_password_missing_special_char() {
        let password = "ValidPass123";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_special");
    }

    #[test]
    fn test_password_with_only_alphanumeric() {
        let password = "ValidPass123";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_special");
    }

    // Edge cases
    #[test]
    fn test_password_only_digits() {
        let password = "12345678";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_lowercase");
    }

    #[test]
    fn test_password_only_uppercase() {
        let password = "ABCDEFGH";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_lowercase");
    }

    #[test]
    fn test_password_only_lowercase() {
        let password = "abcdefgh";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_uppercase");
    }

    #[test]
    fn test_password_only_special_chars() {
        let password = "@$!%*?&@";
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_lowercase");
    }

    // Multiple requirements missing (tests the first error returned)
    #[test]
    fn test_password_missing_multiple_requirements() {
        let password = "short"; // Too short, missing uppercase, digit, special
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_length"); // Length is checked first
    }

    #[test]
    fn test_password_long_but_missing_requirements() {
        let password = "verylongpassword"; // Long enough but missing uppercase, digit, special
        let result = validate_password(password);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "password_uppercase");
    }

    // Boundary tests for allowed special characters
    #[test]
    fn test_all_allowed_special_chars_in_one_password() {
        let password = "ValidPass123@$!%*?&";
        let result = validate_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_with_mix_of_allowed_chars() {
        let password = "MyP@ssw0rd123!";
        let result = validate_password(password);
        assert!(result.is_ok());
    }
}
