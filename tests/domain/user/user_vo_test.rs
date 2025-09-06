use universeg_api::domain::user::vo::{Email, PasswordHash, Username};

#[test]
fn email_parse_ok() {
    assert!(Email::parse("a@b.com").is_ok());
}

#[test]
fn email_parse_err() {
    assert!(Email::parse("invalid").is_err());
}

#[test]
fn username_rules() {
    assert!(Username::parse("valid_name").is_ok());
    assert!(Username::parse("x").is_err()); // según tus reglas
}

#[test]
fn passwordhash_fake_available() {
    let _ = PasswordHash::from_hash(
        "$2b$12$C6UzMDM.H6dfI/f/IKcEe.u9F7c/F7kh/3Gzdh0dX8GZFOD4oTi2.".to_string(),
    )
    .unwrap();
}
