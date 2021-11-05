use crate::PullRequestData;

pub fn json(prs: &Vec<PullRequestData>) -> String {
    serde_json::to_string(prs).unwrap()
}