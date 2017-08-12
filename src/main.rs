extern crate qml;
use qml::*;

extern crate rodio;

mod types;
use types::*;

fn main() {
    let sound_manager = QSoundManager::new(SoundManager{is_playing: false, volume: 0.8, playing: Song{}, sink: None});
    let mut stations_list_model = QStationModel::new();
    stations_list_model.append_row("Station #1".into());
    stations_list_model.append_row("Station #2".into());
    stations_list_model.append_row("Station #3".into());
    let mut engine = QmlEngine::new();
    engine.set_and_store_property("stationListModel", &stations_list_model);
    engine.set_and_store_property("sound_manager", sound_manager.get_qobj());
    engine.load_file("./qml/cantus/main.qml");
    engine.exec();
}
