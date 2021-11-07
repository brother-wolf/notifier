use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequest {
    pub url: String,
    pub state: String,
    pub user: User,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub requested_reviewers: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub ssh_url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
