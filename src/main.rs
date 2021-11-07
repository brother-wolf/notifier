mod files;
mod auth;
mod errors;
mod formats;
mod apis;
mod models;

use structopt::StructOpt;
use tokio::runtime::Runtime;
use crate::formats::json_repo;

#[macro_use]
extern crate log;


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

    // , filter: Vec<&str>
    let requests = rt.block_on(async { apis::github::get_all_repos(&owner, &auth).await });
    match format.as_str() {
        "bitbar" => println!("{:?}", json_repo(&requests)),
        _ => println!("{:?}", json_repo(&requests)),
    }
}
