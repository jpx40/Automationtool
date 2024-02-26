#[derive(Debug, Clone)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            token: None,
            username: Some(username),
            password: Some(password),
        }
    }
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}
