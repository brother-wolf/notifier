use crate::auth::Auth;
use futures::future;
use crate::models::github::{PullRequest, Repo};

pub async fn get_all_repos(owner: &str, auth: &Auth) -> Vec<Repo> {
    let client = reqwest::Client::new();
    let urls = (1..50).map(|page| format!("https://api.github.com/orgs/{}/repos?page={}", owner, page)).collect::<Vec<String>>();
    let bodies = future::join_all(urls.into_iter().map(|url| get_page_of_repos(auth, url, &client))).await;
    bodies.iter()
        .flat_map(|v| v.iter().map(|r| r.clone()))
        .collect::<Vec<Repo>>()
}

async fn get_page_of_repos(auth: &Auth, url: String, client: &reqwest::Client) -> Vec<Repo> {
    match client
        .get(url)
        .basic_auth(auth.user.clone(), Some(auth.token.clone()))
        .header("user-agent", "rust")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.expect("body failed");
                // println!("({}): {}", page, &body);
                let mpr: Result<Vec<Repo>, serde_json::error::Error> = serde_json::from_str(&body);
                match mpr {
                    Ok(pr) => pr,
                    Err(e) => {
                        error!("{}", e);
                        vec![]
                    }
                }
            } else {
                let status_code = &response.status().as_u16();
                let body = &response.text().await;
                error!("Code: {}, body: {}", status_code, body.as_ref().unwrap());
                vec![]
            }
        }
        Err(error) => {
            error!("failed to get pull requests: {}", error.to_string());
            vec![]
        }
    }
}

pub(crate) async fn get_pull_requests(owner: &str, repo: &str, state: &str, auth: &Auth) -> Vec<PullRequest> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls?state={}", owner, repo, state);
    match reqwest::Client::new()
        .get(url)
        .basic_auth(auth.user.clone(), Some(auth.token.clone()))
        .header("user-agent", "rust")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.expect("body failed");
                let mpr: Result<Vec<PullRequest>, serde_json::error::Error> = serde_json::from_str(&body);
                match mpr {
                    Ok(pr) => pr,
                    Err(e) => {
                        error!("{}", e);
                        vec![]
                    }
                }
            } else {
                let status_code = &response.status().as_u16();
                let body = &response.text().await;
                error!("Code: {}, body: {}", status_code, body.as_ref().unwrap());
                vec![]
            }
        }
        Err(error) => {
            error!("failed to get pull requests: {}", error.to_string());
            vec![]
        }
    }
}