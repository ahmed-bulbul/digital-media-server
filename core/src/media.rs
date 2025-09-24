use crate::errors::MediaError;

pub struct MediaStream {
    pub id: u32,
    pub title: String,
    pub duration: f64,
}

impl MediaStream {
    pub fn play(&self) -> Result<(), MediaError> {
        if self.title.to_lowercase().contains("invalid") {
            Err(MediaError::InvalidFormat)
        } else {
            println!("Playing {} [{}s]", self.title, self.duration);
            Ok(())
        }
    }
}
