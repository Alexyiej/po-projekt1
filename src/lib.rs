pub mod utils {
    pub mod constants;
}


pub mod database {
    pub mod conn;

    pub mod queries{
        pub mod auth;
        pub mod profile;
        pub mod family;
    }
}


pub mod state;