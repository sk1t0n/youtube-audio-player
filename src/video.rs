use std::process::exit;

use rustube::{url::Url, Error, Id, IdBuf, Stream, Video};

pub async fn get_audio_stream_url(video_id: String) -> String {
    let id: IdBuf = get_id(video_id);
    let video: Video = get_video(id).await;
    let url: &Url = get_url(&video);

    let domain: Option<&str> = url.domain();

    let domain: &str = match domain {
        Some(v) => v,
        None => {
            println!("Error: failed to get the url.");
            exit(1);
        }
    };

    let path: &str = url.path();
    let query: Option<&str> = url.query();

    let query: &str = match query {
        Some(v) => v,
        None => {
            println!("Error: failed to get the url.");
            exit(1);
        }
    };

    let audio_stream_url: String = format!("https://{domain}{path}?{query}");

    audio_stream_url
}

fn get_id(video_id: String) -> IdBuf {
    let url: String = format!("https://www.youtube.com/watch?v={video_id}");
    let id: Result<Id, Error> = Id::from_raw(&url);

    let id: Id = match id {
        Ok(id) => id,
        Err(_) => {
            println!("Error: the video ID is incorrect.");
            exit(0);
        }
    };

    id.into_owned()
}

async fn get_video(id: IdBuf) -> Video {
    let video: Result<Video, Error> = Video::from_id(id).await;

    match video {
        Ok(v) => return v,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };
}

fn get_url(video: &Video) -> &Url {
    let audio_stream: Option<&Stream>;
    let is_worst_quality: bool = true;

    if is_worst_quality {
        audio_stream = video.worst_audio();
    } else {
        audio_stream = video.best_audio();
    }

    let audio_stream: &Stream = match audio_stream {
        Some(v) => v,
        None => {
            println!("Error: the url is empty.");
            exit(1);
        }
    };

    let url: &Url = &audio_stream.signature_cipher.url;

    url
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_id() {
        let video_id = String::from("kUft3eH1fy0");
        super::get_id(video_id);
    }
}
