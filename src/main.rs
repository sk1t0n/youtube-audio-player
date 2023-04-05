use std::env;

pub mod player;
pub mod video;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Error: the video ID has not been set.");
        return;
    }

    let video_id: &str = &args[1];
    let url: String = video::get_audio_stream_url(video_id).await;

    player::play(url);
}
