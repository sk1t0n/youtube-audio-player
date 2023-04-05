use std::{
    io::Error,
    process::{Command, Output},
};

/// Starts the VLC media player using the received url.
pub fn play(url: String) {
    let result: Result<Output, Error> = Command::new("vlc").arg(url).output();

    match result {
        Ok(_) => return,
        Err(e) => {
            if e.to_string().starts_with("No such file or directory") {
                println!("Error: the VLC media player was not found.");
            } else {
                println!("Error: {e}");
            }
        }
    }
}