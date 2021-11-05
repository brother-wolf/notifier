mod files;
mod auth;
mod errors;
mod formats;
mod apis;

use serde::Deserialize;
use serde::Serialize;
use structopt::StructOpt;
use tokio::runtime::Runtime;
use formats::json;

#[macro_use]
extern crate log;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    login: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestData {
    url: String,
    state: String,
    user: User,
    body: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
    merged_at: Option<String>,
    requested_reviewers: Vec<User>,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "a", long = "auth-file", required = true)]
    auth_file: String,
    #[structopt(short = "f", long = "format", default_value = "json", possible_values = & vec ! [ "json", "bitbar" ], help = "The format for the output")]
    format: String,
    #[structopt(short = "o", long = "owner", required = true)]
    owner: String,
    #[structopt(short = "r", long = "repo", required = true)]
    repo: String,

}

fn main() {
    let rt = Runtime::new().unwrap();
    let opt = Opt::from_args();
    let auth_file: String = opt.auth_file;
    let owner: String = opt.owner;
    let repo: String = opt.repo;
    let format: String = opt.format;
    let auth = auth::Auth::from_json(&files::lines_from_file(auth_file).expect("Could not load lines").join("\n"));
    let requests = rt.block_on(async { apis::github::get_pull_requests(&owner, &repo, "open", &auth).await });
    match format.as_str() {
        "bitbar" => println!("{:?}", json(&requests)),
        _ => println!("{:?}", json(&requests)),
    }
}
