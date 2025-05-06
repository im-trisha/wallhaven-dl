use clap::Parser;
use wallhaven_api::Resolutions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to which write the images
    #[arg(short, long, default_value = "~/Pictures/DownloadedWallpapers")]
    pub outdir: String,
    /// The api key that needs to be used. If not present, tries using WALLHAVEN_API_KEY, or else defaults to none
    #[arg(short, long)]
    pub api_key: Option<String>,
    /// Image output resolution. Your problem to know which ones are possible, not mine tbh
    #[arg(short, long, value_parser=parse_resolution, default_value="1920x1080")]
    pub resolution: Option<Resolutions>,
    /// Whether you want to remove the downloaded tabs from firefox
    #[arg(long)]
    pub remove: bool,
}

fn parse_resolution(s: &str) -> Result<Resolutions, String> {
    serde_json::from_str(&format!("\"{}\"", s)).map_err(|e| e.to_string())
}
