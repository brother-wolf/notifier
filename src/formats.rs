use crate::models::github::{PullRequest, Repo};

pub fn json(prs: &Vec<PullRequest>) -> String {
    serde_json::to_string(prs).unwrap()
}

pub fn json_repo(prs: &Vec<Repo>) -> String {
    serde_json::to_string(prs).unwrap()
}