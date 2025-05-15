use crate::constants::{BRANCH, OWNER, REPO, WORKFLOW};
use http::header;
use serde_json::Value;
use std::time::Duration;
use ureq::Agent;

/// Fetches the latest workflow artifact archive and corresponding commit using
/// the [`OWNER`], [`REPO`], [`WORKFLOW`] and [`BRANCH`] constants.
///
/// Requires a `GITHUB_TOKEN` environment variable to download the artifact archive.
pub fn get_archive_and_commit() -> (Vec<u8>, String) {
    let token = format!("Bearer {}", dotenvy::var("GITHUB_TOKEN").unwrap());

    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .user_agent("tigerros/reco")
        .build()
        .into();

    let runs_url = format!(
        "https://api.github.com/repos/{OWNER}/{REPO}/actions/workflows/{WORKFLOW}/runs?branch={BRANCH}"
    );

    let runs_res: Value = agent
        .get(runs_url)
        .header(header::AUTHORIZATION, &token)
        .call()
        .unwrap()
        .into_body()
        .read_json()
        .unwrap();

    let mut archive_download_url_and_commit_sha = None::<(String, String)>;

    for run in runs_res["workflow_runs"].as_array().unwrap() {
        if run["name"] != "Lint" || run["conclusion"] != "success" || run["event"] != "push" {
            continue;
        }

        let artifacts_url = run["artifacts_url"].as_str().unwrap();

        let artifacts_res: Value = agent
            .get(artifacts_url)
            .header(header::AUTHORIZATION, &token)
            .call()
            .unwrap()
            .into_body()
            .read_json()
            .unwrap();

        if let Some(artifact) = artifacts_res["artifacts"].as_array().unwrap().first() {
            archive_download_url_and_commit_sha = Some((
                artifact["archive_download_url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                run["head_sha"].as_str().unwrap().to_owned(),
            ));
            break;
        };
    }

    let Some((archive_download_url, commit_sha)) = archive_download_url_and_commit_sha else {
        panic!("no artifact found");
    };

    let archive_res = agent
        .get(archive_download_url)
        .header(header::AUTHORIZATION, &token)
        .call()
        .unwrap()
        .into_body()
        .into_with_config()
        .limit(u64::MAX)
        .read_to_vec()
        .unwrap();

    (archive_res, commit_sha)
}
