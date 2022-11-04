use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

// Global variables for TRACKS
const TRACKS: [&str; 7] = [
    "wracajac.mp3",
    "wracajac2.mp3",
    "austria.mp3",
    "kaczka.mp3",
    "mistrze_swiata.mp3",
    "nitek_drogi.mp3",
    "sugerujesz_insynuacje.mp3",
];
const TRACKS_LEN: i32 = TRACKS.len() as i32;

// Plays given audio
fn play_audio(file_name: &str) {
    // Load output stream and sink for multithreaded audio
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file
    let file = File::open("./src/tracks/".to_owned() + file_name).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Add audio to sink and play it until it ends
    sink.append(source);
    sink.sleep_until_end();
}

// Displays TRACKS and info
fn help() {
    println!("-------------TRACKS-------------");
    for i in 0..TRACKS.len() {
        println!("{} => {}", i + 1, TRACKS[i]);
    }
    println!("0 => EXIT");
    println!("-1 => HELP");
}

// Interaction with user
fn choose_track() {
    help();
    loop {
        // Get input from user and convert to i32
        println!("Choose a track");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Incorrect input");
        let value: i32 = input.trim().parse().unwrap_or(-1);

        // Match the given input to a file name
        match value {
            -1 => help(),
            0 => return,
            1..=TRACKS_LEN => play_audio(TRACKS[value as usize - 1]),
            _ => println!("error"),
        }
    }
}
fn main() -> std::io::Result<()> {
    choose_track();
    Ok(())
}
