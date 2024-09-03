// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use models::TrustyResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: TrustyResponse = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrustyResponse {
    id: Option<String>,

    status: Option<String>,

    status_code: Option<serde_json::Value>,

    name: Option<String>,

    #[serde(rename = "type")]
    ty: Option<String>,

    version: Option<String>,

    version_date: Option<String>,

    author: Option<String>,

    author_email: Option<String>,

    package_description: Option<String>,

    repo_description: Option<String>,

    origin: Option<String>,

    stargazers_count: Option<i64>,

    watchers_count: Option<i64>,

    home_page: Option<String>,

    has_issues: Option<bool>,

    has_projects: Option<bool>,

    has_downloads: Option<serde_json::Value>,

    forks_count: Option<i64>,

    archived: Option<bool>,

    is_deprecated: Option<bool>,

    disabled: Option<bool>,

    open_issues_count: Option<i64>,

    visibility: Option<String>,

    default_branch: Option<String>,

    repository_id: Option<String>,

    repository_name: Option<String>,

    contributor_count: Option<i64>,

    public_repos: Option<i64>,

    public_gists: Option<i64>,

    followers: Option<i64>,

    following: Option<i64>,

    owner: Option<Owner>,

    contributors: Option<Vec<Owner>>,

    last_update: Option<String>,

    scores: Option<Scores>,

    malicious: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    id: Option<String>,

    author: Option<String>,

    author_email: Option<String>,

    login: Option<String>,

    avatar_url: Option<String>,

    gravatar_id: Option<String>,

    url: Option<String>,

    html_url: Option<String>,

    company: Option<String>,

    blog: Option<String>,

    location: Option<String>,

    email: Option<String>,

    hireable: Option<bool>,

    twitter_username: Option<String>,

    public_repos: Option<i64>,

    public_gists: Option<serde_json::Value>,

    followers: Option<i64>,

    following: Option<i64>,

    scores: Option<Scores>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scores {
    // Add fields here if needed, all wrapped in Option<T>
}