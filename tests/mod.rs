mod common;

mod interface {
    mod user {
        mod get_info;
    }
    mod auth {
        mod login;
        mod login_dto_tests;
        mod password_validation_tests;
        mod register;
        mod register_dto_tests;
    }
}
mod domain {
    mod user {
        mod user_test;
        mod user_vo_test;
    }
}
