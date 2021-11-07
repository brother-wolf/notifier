use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub user: String,
    pub token: String,
}

impl Auth {
    pub fn from_json(json: &str) -> Auth {
        serde_json::from_str(json).unwrap()
    }
}

#[test]
fn should_extract_auth_from_json() {
    let json = r#"{"user": "wibble", "token": "wobble"}"#;
    // let auth = crate::config::Config::load::<Auth>(json)?;
    let auth = Auth::from_json(json);
    assert_eq!("wibble", auth.user);
    assert_eq!("wobble", auth.token);
}