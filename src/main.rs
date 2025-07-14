use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;

#[derive(Parser)]
#[command(name = "mp3-converter")]
#[command(about = "A CLI tool to convert MP4 to MP3 and download YouTube videos as MP3")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(short, long, help = "Input MP4 file path")]
        input: PathBuf,
        #[arg(short, long, help = "Output MP3 file path (optional)")]
        output: Option<PathBuf>,
    },
    Download {
        #[arg(short, long, help = "YouTube URL")]
        url: String,
        #[arg(short, long, help = "Output directory (optional)")]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, output } => {
            convert_mp4_to_mp3(input, output).await?
        }
        Commands::Download { url, output } => {
            download_youtube_as_mp3(url, output).await?
        }
    }

    Ok(())
}

async fn convert_mp4_to_mp3(
    input: PathBuf,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file does not exist: {:?}", input).into());
    }

    let output_path = match output {
        Some(path) => path,
        None => {
            let mut path = input.clone();
            path.set_extension("mp3");
            path
        }
    };

    println!("Converting {:?} to {:?}...", input, output_path);

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&input)
        .arg("-vn")
        .arg("-acodec")
        .arg("mp3")
        .arg("-ab")
        .arg("192k")
        .arg("-ar")
        .arg("44100")
        .arg("-y")
        .arg(&output_path)
        .status()?;

    if status.success() {
        println!("✅ Successfully converted to {:?}", output_path);
    } else {
        return Err("FFmpeg conversion failed".into());
    }

    Ok(())
}

async fn download_youtube_as_mp3(
    url: String,
    output_dir: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = output_dir.unwrap_or_else(|| PathBuf::from("."));

    if !output_path.exists() {
        fs::create_dir_all(&output_path).await?;
    }

    println!("Downloading and converting {} to MP3...", url);

    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--audio-quality")
        .arg("192K")
        .arg("-o")
        .arg(format!(
            "{}/%(title)s.%(ext)s",
            output_path.to_string_lossy()
        ))
        .arg(&url)
        .status()?;

    if status.success() {
        println!("✅ Successfully downloaded and converted to MP3 in {:?}", output_path);
    } else {
        return Err("yt-dlp download failed".into());
    }

    Ok(())
}
