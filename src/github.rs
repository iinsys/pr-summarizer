use std::str::FromStr;

use anyhow::{Context, Result};
use octocrab::Octocrab;

#[allow(dead_code)]
pub struct PrInfo {
    pub title: String,
    pub description: String,
    pub base_branch: String,
    pub head_branch: String,
    pub author: String,
    pub changed_files: Vec<ChangedFile>,
}

#[allow(dead_code)]
pub struct ChangedFile {
    pub filename: String,
    pub status: FileStatus,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug)]
pub enum FileStatus {
    Added,
    Modified,
    Removed,
    Renamed,
}

impl FromStr for FileStatus {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "added" => Ok(FileStatus::Added),
            "modified" => Ok(FileStatus::Modified),
            "removed" => Ok(FileStatus::Removed),
            "renamed" => Ok(FileStatus::Renamed),
            _ => Ok(FileStatus::Modified),
        }
    }
}

pub fn create_github_client(token: &str) -> Result<Octocrab> {
    Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .context("Failed to build GitHub client")
}

pub async fn get_pr_info(
    client: &Octocrab,
    owner: &str,
    repo: &str,
    pr_number: u64,
) -> Result<PrInfo> {
    // Get PR details
    let pr = client
        .pulls(owner, repo)
        .get(pr_number)
        .await
        .context("Failed to get PR details")?;

    // Get PR changed files
    let files = client
        .pulls(owner, repo)
        .list_files(pr_number)
        .await
        .context("Failed to get PR files")?;

    // Convert to our internal model
    let changed_files = files
        .into_iter()
        .map(|file| {
            let status = format!("{:?}", file.status);
            let status = FileStatus::from_str(&status).unwrap();
            let additions = file.additions as i32;
            let filename = file.filename;
            let deletions = file.deletions as i32;
            ChangedFile {
                filename,
                status,
                additions,
                deletions,
            }
        })
        .collect();

    fn default<T: Default>() -> impl FnOnce() -> T {
        || T::default()
    }
    Ok(PrInfo {
        title: pr.title.unwrap_or_default(),
        description: pr.body.unwrap_or_default(),
        base_branch: pr
            .base
            .repo
            .as_ref()
            .map_or_else(default::<String>(), |repo| {
                repo.owner
                    .as_ref()
                    .map_or_else(default::<String>(), |owner| owner.login.clone())
            }),
        head_branch: pr.head.ref_field,
        author: pr.user.unwrap().login,
        changed_files,
    })
}

pub async fn post_pr_comment(
    client: &Octocrab,
    owner: &str,
    repo: &str,
    pr_number: u64,
    comment: &str,
) -> Result<()> {
    client
        .issues(owner, repo)
        .create_comment(pr_number, comment)
        .await
        .context("Failed to post comment")?;
    Ok(())
}
