use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

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

// Displays tracks and info
fn help(tracks: [&str; 7]) {
    println!("-------------tracks-------------");
    for i in 0..tracks.len() {
        println!("{} => {}", i + 1, tracks[i]);
    }
    println!("0 => EXIT");
    println!("-1 => HELP");
}

// Interaction with user
fn choose_track(tracks: [&str; 7]) {
    help(tracks);
    loop {
        // Get input from user and convert to i32
        println!("Choose a track");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Incorrect input");
        let value: i32 = input.trim().parse().expect("Not a number");

        // Match the given input to a file name
        match value {
            -1 => help(tracks),
            0 => return,
            1 => play_audio(tracks[value as usize - 1]),
            2 => play_audio(tracks[value as usize - 1]),
            3 => play_audio(tracks[value as usize - 1]),
            4 => play_audio(tracks[value as usize - 1]),
            5 => play_audio(tracks[value as usize - 1]),
            6 => play_audio(tracks[value as usize - 1]),
            7 => play_audio(tracks[value as usize - 1]),
            _ => println!("error"),
        }
    }
}
fn main() -> std::io::Result<()> {
    // Names of tracks
    let tracks: [&str; 7] = [
        "wracajac.mp3",
        "wracajac2.mp3",
        "austria.mp3",
        "kaczka.mp3",
        "mistrze_swiata.mp3",
        "nitek_drogi.mp3",
        "sugerujesz_insynuacje.mp3",
    ];
    choose_track(tracks);
    Ok(())
}
