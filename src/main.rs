use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf};

mod solutions;

fn get_session() -> Result<String, Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let session_file = home.join(".aoc");
    std::fs::read_to_string(session_file)
        .map(|s| s.trim().to_owned())
        .map_err(From::from)
}

async fn download_input(year: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let session = get_session()?;
    let client = reqwest::Client::builder().build()?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let res = client
        .get(&url)
        .header("cookie", format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    std::fs::create_dir_all(format!("{}/{}", year, day))?;
    std::fs::write(get_input_location(year, day), res)?;
    Ok(())
}

fn get_input_location(year: &str, day: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}/input", year, day))
}

async fn run_input(year: &str, day: &str, part: &str) -> Result<(), Box<dyn Error>> {
    let input_path = get_input_location(year, day);
    if !input_path.exists() {
        download_input(year, day).await?;
    }
    let input = std::fs::read_to_string(input_path)?;
    let res = solutions::run(year, day, part == "1", &input)?;
    println!("Result is {}", res);
    Ok(())
}

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Input {
        year: String,
        day: String,
    },
    Run {
        year: String,
        day: String,
        part: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Input { year, day } => download_input(&year, &day).await?,
        Command::Run { year, day, part } => {
            run_input(&year, &day, &part.unwrap_or("1".into())).await?
        }
    }
    Ok(())
}
