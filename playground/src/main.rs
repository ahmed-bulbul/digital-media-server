use core::media::MediaStream;
use core::core_info;

fn main() {
    core_info();

    let valid_stream = MediaStream {
        id: 1,
        title: "Rust Demo".to_string(),
        duration: 99.0,
    };

    let invalid_stream = MediaStream {
        id: 2,
        title: "invalid video".to_string(),
        duration: 42.0,
    };

    // Play valid stream
    match valid_stream.play() {
        Ok(_) => println!("Played valid stream successfully."),
        Err(e) => println!("Error playing valid stream: {}", e),
    }

    // Play invalid stream
    match invalid_stream.play() {
        Ok(_) => println!("Played invalid stream successfully."),
        Err(e) => println!("Error playing invalid stream: {}", e),
    }
}
