use crate::auth::Auth;
use crate::PullRequestData;

pub(crate) async fn get_pull_requests(owner: &str, repo: &str, state: &str, auth: &Auth) -> Vec<PullRequestData> {
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
                let mpr: Result<Vec<PullRequestData>, serde_json::error::Error> = serde_json::from_str(&body);
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