use log::info;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<(), String> {
    let stdout = Stdio::inherit();

    let exit_status = Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        return Err(format!(
            "git exited with error code: {:?}",
            exit_status.code()
        ));
    }

    Ok(())
}

pub fn clone_repo(repo: &str, dir: &str, rev: &str) {
    let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);

    info!("Cloning {} repo...", dir.split('/').nth(2).unwrap());

    run_git(["clone", repo, full_path.to_str().unwrap()]).expect("failed to clone");
    run_git(["-C", full_path.to_str().unwrap(), "checkout", rev]).expect("failed to checkout");
}

#[derive(Debug, PartialEq)]
pub enum Module {
    CosmosSdk,
    Connect,
}

pub fn get_version_from_go_mod(project_dir: &str, module: Module) -> Result<String, String> {
    let go_mod_path = Path::new(project_dir).join("go.mod");
    let content = fs::read_to_string(go_mod_path).expect("Failed to read go.mod file");

    match module {
        Module::CosmosSdk => {
            extract_module_version(&content, "cosmos/cosmos-sdk", "MANTRA-Chain/cosmos-sdk")
        }
        Module::Connect => {
            extract_module_version(&content, "skip-mev/connect/v2", "MANTRA-Chain/connect/v2")
        }
    }
}

fn extract_module_version(
    content: &str,
    original_module: &str,
    replacement_module: &str,
) -> Result<String, String> {
    let pattern = format!(
        r#"github.com/{}\s+=>\s+github.com/{}\s+([^\s]+)"#,
        original_module, replacement_module
    );

    let re = Regex::new(&pattern).map_err(|_| "Failed to create regex pattern".to_string())?;

    if let Some(captures) = re.captures(content) {
        let version = captures
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or("Failed to extract version".to_string())?;

        // Check if version follows the pattern: {timestamp}-{commit_hash}
        if let [commit_hash, timestamp, ..] = version.split('-').rev().collect::<Vec<&str>>()[..] {
            let is_valid_timestamp = timestamp.len() >= 14
                && timestamp[timestamp.len() - 14..]
                    .chars()
                    .all(|c| c.is_ascii_digit());
            let is_valid_commit =
                commit_hash.len() == 12 && commit_hash.chars().all(|c| c.is_alphanumeric());

            if is_valid_timestamp && is_valid_commit {
                return Ok(commit_hash.to_string());
            }
        }

        // Return the version as is (for tag-style versions)
        Ok(version)
    } else {
        Err(format!(
            "Failed to find {} version in go.mod",
            original_module
        ))
    }
}
