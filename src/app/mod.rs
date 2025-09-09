pub mod ports;
pub mod usecases {
    pub mod auth {
        pub mod login;
        pub mod register;
        pub mod email_ver;
    }
    pub mod user {
        pub mod get_info;
    }
    pub mod mail {
        pub mod send;
    }
}
