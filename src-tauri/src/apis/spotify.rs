use librespot::discovery::Credentials;

pub fn password_login(username: &str, password: &str) -> Credentials {
    Credentials::with_password(username, password)
}
