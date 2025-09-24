// use core::media::MediaStream;
//
// fn main() {
//     let stream = MediaStream {
//         id: 42,
//         title: "Rust Demo".to_string(),
//         duration: 99.0,
//     };
//     println!("Loaded {} [{}s]", stream.title, stream.duration);
// }


trait Playable {
    fn play(&self);
}

struct Song {
    title: String,
}

impl Playable for Song {
    fn play(&self) {
        println!("Playing song: {}", self.title);
    }
}

fn main() {
    let s = Song { title: "Rust Beats".into() };
    s.play();
}
