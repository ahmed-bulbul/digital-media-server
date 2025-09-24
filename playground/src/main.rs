use core::media::MediaStream;

fn main() {
    let stream = MediaStream {
        id: 42,
        title: "Rust Demo".to_string(),
        duration: 99.0,
    };
    println!("Loaded {} [{}s]", stream.title, stream.duration);
}
