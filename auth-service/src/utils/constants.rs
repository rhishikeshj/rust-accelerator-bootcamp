use dotenvy::dotenv;
use std::sync::LazyLock;

pub const JWT_COOKIE_NAME: &str = "jwt";
pub static JWT_SECRET: LazyLock<String> = LazyLock::new(set_token);

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std::env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    // assign the port randomly for each test
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
