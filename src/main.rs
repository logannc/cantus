extern crate qml;

use qml::*;

fn main() {
    println!("Hello, world!");
    let mut engine = QmlEngine::new();
    engine.load_file("./qml/cantus/main.qml");
    engine.exec();
}
