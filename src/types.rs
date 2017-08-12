use qml::*;
use rodio;
use rodio::{Sink};
use std;

Q_LISTMODEL!{
    pub QStationModel {
        station_name: String
    }
}

pub struct Song;
pub struct SoundManager {
    pub is_playing: bool,
    pub volume: f32,
    pub playing: Song,
    pub sink: Option<Sink>
}
Q_OBJECT!{
    pub SoundManager as QSoundManager {
        signals:
        slots:
            fn toggle_play();
            fn volume_change(vol: f32);
            fn next_song();
        properties:
    }
}

impl QSoundManager {
    fn toggle_play(&mut self) -> Option<&QVariant> {
        if self.sink.is_none() {
            let ep = rodio::get_default_endpoint().unwrap();
            let sink = rodio::Sink::new(&ep);
            self.sink = Some(sink);
        }
        if self.is_playing {
            println!("Pausing...");
            self.sink.as_ref().unwrap().pause();
        } else {
            let nv = self.volume;
            println!("Setting sink volume to {}...", nv);
            self.sink.as_mut().unwrap().set_volume(nv);
            println!("Playing...");
            self.sink.as_ref().unwrap().play();
        }
        self.is_playing = ! self.is_playing;
        None
    }
    fn volume_change(&mut self, vol: f32) -> Option<&QVariant> {
        if self.sink.is_none() {
            let ep = rodio::get_default_endpoint().unwrap();
            let sink = rodio::Sink::new(&ep);
            self.sink = Some(sink);
        }
        self.volume = vol;
        self.sink.as_mut().unwrap().set_volume(vol);
        println!("Volume set to {}", self.volume);
        None
    }
    fn next_song(&mut self) -> Option<&QVariant> {
        if self.sink.is_none() {
            let ep = rodio::get_default_endpoint().unwrap();
            let sink = rodio::Sink::new(&ep);
            self.sink = Some(sink);
        }
        self.sink.as_ref().unwrap().stop();
        let p = std::path::Path::new("./ftabtr.ogg");
        if p.exists() {
            println!("Adding a song!");
            let file = std::fs::File::open("./ftabtr.ogg").unwrap();
            let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
            self.sink.as_mut().unwrap().append(source);
        } else {
            println!("File doesn't exist...");
        }
        None
    }
}