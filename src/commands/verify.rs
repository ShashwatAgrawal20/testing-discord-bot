use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

#[derive(Serialize, Deserialize, Debug)]
struct GitHubUser {
    login: String,
    id: u64,
    avatar_url: String,
    html_url: String,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    bio: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
}

async fn fetch_github_user(username: &str) -> Result<GitHubUser> {
    let url = format!("https://api.github.com/users/{}", username);

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "reqwest")
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let user: GitHubUser = serde_json::from_str(&body)?;

        // println!("{:#?}", user);
        Ok(user)
    } else {
        anyhow::bail!("FU");
    }
}

pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(user),
        ..
    }) = options.first()
    {
        match fetch_github_user(user).await {
            Ok(profile) => format!(
                "GitHub User: {}\nName: {}\nBio: {}\nPublic Repos: {}\nFollowers: {}\nFollowing: {}\nProfile: {}",
                profile.login,
                profile.name.unwrap_or_else(|| "N/A".to_string()),
                profile.bio.unwrap_or_else(|| "N/A".to_string()),
                profile.public_repos,
                profile.followers,
                profile.following,
                profile.html_url
            ),
            Err(_) => "Failed to fetch GitHub user information.".to_string(),
        }
    } else {
        "Invalid input. Please provide a GitHub username.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("verify")
        .description("Get some GitHub information")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "string", "GitHub Username")
                .required(true),
        )
}
