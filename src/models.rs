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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrustyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    ty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    version_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    package_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repo_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stargazers_count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    watchers_count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    home_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    has_issues: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    has_projects: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    has_downloads: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    forks_count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    open_issues_count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contributor_count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_repos: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_gists: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    followers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    following: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contributors: Option<Vec<Owner>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scores: Option<Scores>,

    #[serde(skip_serializing_if = "Option::is_none")]
    malicious: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gravatar_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blog: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hireable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    twitter_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_repos: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_gists: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    followers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    following: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scores: Option<Scores>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scores {
    // Add fields here if needed, all wrapped in Option<T>
}
