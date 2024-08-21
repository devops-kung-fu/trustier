// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::TrustyResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: TrustyResponse = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrustyResponse {
    id: String,

    status: String,

    status_code: Option<serde_json::Value>,

    name: String,

    #[serde(rename = "type")]
    trusty_response_type: String,

    version: String,

    version_date: String,

    author: String,

    author_email: String,

    package_description: String,

    repo_description: String,

    origin: String,

    stargazers_count: i64,

    watchers_count: i64,

    home_page: String,

    has_issues: bool,

    has_projects: bool,

    has_downloads: bool,

    forks_count: i64,

    archived: bool,

    is_deprecated: bool,

    disabled: bool,

    open_issues_count: i64,

    visibility: String,

    default_branch: String,

    repository_id: String,

    repository_name: String,

    contributor_count: i64,

    public_repos: i64,

    public_gists: i64,

    followers: i64,

    following: i64,

    owner: Owner,

    contributors: Vec<Owner>,

    last_update: String,

    scores: Scores,

    malicious: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    id: String,

    author: String,

    author_email: String,

    login: String,

    avatar_url: String,

    gravatar_id: String,

    url: String,

    html_url: String,

    company: Option<String>,

    blog: Option<String>,

    location: Option<String>,

    email: String,

    hireable: bool,

    twitter_username: Option<serde_json::Value>,

    public_repos: i64,

    public_gists: Option<serde_json::Value>,

    followers: i64,

    following: i64,

    scores: Scores,
}

#[derive(Serialize, Deserialize)]
pub struct Scores {}
