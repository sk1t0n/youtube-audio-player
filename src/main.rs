use clap::Parser;

pub mod player;
pub mod video;

#[derive(Parser)]
#[command()]
struct Args {
    /// the ID of the YouTube video you want
    #[arg()]
    video_id: String,

    /// the VLC start mode: 1 - GUI, 2 - terminal
    #[arg(short, default_value_t = 1)]
    mode: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let video_id = args.video_id;
    let mode = args.mode;
    let url: String = video::get_audio_stream_url(video_id).await;

    player::play(url, mode);
}
