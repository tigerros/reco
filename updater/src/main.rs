use std::io::Cursor;
use std::time::Duration;
use http::header;
use serde_json::Value;
use ureq::Agent;
use zip::ZipArchive;

const OWNER: &str = "lichess-org";
const REPO: &str = "chess-openings";
const WORKFLOW: &str = "lint.yml";

fn main() -> anyhow::Result<()> {
    let token = format!("Bearer {}", dotenvy::var("GITHUB_TOKEN")?);

    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .user_agent("tigerros/reco")
        .build()
        .into();

    let runs_url = format!(
        "https://api.github.com/repos/{OWNER}/{REPO}/actions/workflows/{WORKFLOW}/runs?per_page=1",
    );

    let runs_res: Value = agent
        .get(runs_url)
        .header(header::AUTHORIZATION, &token)
        .call()?
        .into_body()
        .read_json()?;

    let artifacts_url = runs_res["workflow_runs"][0]["artifacts_url"].as_str().unwrap();

    let artifacts_res: Value = agent
        .get(artifacts_url)
        .header(header::AUTHORIZATION, &token)
        .call()?
        .into_body()
        .read_json()?;

    let archive_download_url = artifacts_res["artifacts"][0]["archive_download_url"].as_str().unwrap();

    let archive_res = agent
        .get(archive_download_url)
        .header(header::AUTHORIZATION, &token)
        .call()?
        .into_body()
        .into_with_config()
        .limit(u64::MAX)
        .read_to_vec()?;

    let mut archive = ZipArchive::new(Cursor::new(archive_res))?;
    let data = archive.by_name("all.tsv")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .flexible(false)
        .from_reader(data);
    
    for result in reader.records() {
        let record = result?;
        
        
    }

    Ok(())
}