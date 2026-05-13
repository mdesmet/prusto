use std::fmt;

#[derive(Clone)]
pub enum Auth {
    Basic(String, Option<String>),
    Bearer(String),
}

impl Auth {
    pub fn new_basic(username: impl ToString, password: Option<impl ToString>) -> Auth {
        Auth::Basic(username.to_string(), password.map(|p| p.to_string()))
    }

    pub fn new_bearer(bearer_token: impl ToString) -> Auth {
        Auth::Bearer(bearer_token.to_string())
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::Basic(name, _) => f
                .debug_struct("BasicAuth")
                .field("username", name)
                .field("password", &"******")
                .finish(),
            Auth::Bearer(_) => f
                .debug_struct("BearerAuth")
                .field("token", &"******")
                .finish(),
        }
    }
}
