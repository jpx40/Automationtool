#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub key: Option<String>,
    pub key_path: Option<String>,

    pub key_phrase: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            token: None,
            username: Some(username),
            password: Some(password),
            key: None,
            key_path: None,
            key_phrase: None,
        }
    }
    pub fn empty() -> User {
        User {
            token: None,
            username: None,
            password: None,
            key: None,
            key_path: None,
            key_phrase: None,
        }
    }

    pub fn with_key(key: String) -> User {
        User {
            token: None,
            key: Some(key),
            password: None,
            username: None,
            key_path: None,
            key_phrase: None,
        }
    }
    fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }
    pub fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}
