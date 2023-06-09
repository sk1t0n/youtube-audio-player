# youtube-audio-player

The application can to play the YouTube audio stream using the VLC media player.

![screenshot](screenshot.png)

## Install

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. [Download VLC](https://www.videolan.org/vlc/)
3. Build the application: `cargo build --release`

## Usage

```sh
# video_id - the ID of the YouTube video you want
./target/release/youtube-audio-player video_id

# Start the VLC media player in the command line mode
./target/release/youtube-audio-player video_id -m 2

# Show the audio stream URL
./target/release/youtube-audio-player video_id -s

# Show help
./target/release/youtube-audio-player -h
```

## License

[MIT](LICENSE)
