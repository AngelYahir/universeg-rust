use universeg_api::domain::user::user::User;
use universeg_api::domain::user::vo::{Email, PasswordHash, Username};
use uuid::Uuid;

#[test]
fn user_new_assigns_fields() {
    let id = Uuid::new_v4();
    let email = Email::parse("dev@universeg.com").unwrap();
    let username = Username::parse("angel").unwrap();
    let pw = PasswordHash::from_hash(
        "$2b$12$C6UzMDM.H6dfI/f/IKcEe.u9F7c/F7kh/3Gzdh0dX8GZFOD4oTi2.".to_string(),
    )
    .unwrap();

    let u = User::new(id, email.clone(), username.clone(), pw.clone(), true);
    assert_eq!(u.id, id);
    assert_eq!(u.email, email);
    assert_eq!(u.username, username);
    assert_eq!(u.password_hash, pw);
    assert!(u.is_email_verified);
}
