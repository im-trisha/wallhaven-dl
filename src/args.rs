use clap::Parser;
use wallhaven_rs::Resolution;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where we write the images to
    #[arg(short, long, default_value = "~/Pictures/Wallpapers/Downloaded")]
    pub outdir: String,
    /// The api key that needs to be used. If not present, tries using WALLHAVEN_API_KEY, or else defaults to none
    #[arg(short, long)]
    pub api_key: Option<String>,
    /// Image output resolution. Your problem to know which ones are possible, not mine tbh
    #[arg(short, long, value_parser=parse_resolution, default_value="1920x1080")]
    pub resolution: Option<Resolution>,
    /// Whether you want to keep the downloaded tabs on firefox
    #[arg(long)]
    pub no_remove: bool,
}

fn parse_resolution(s: &str) -> serde_json::Result<Resolution> {
    serde_json::from_str(&format!("\"{s}\""))
}
