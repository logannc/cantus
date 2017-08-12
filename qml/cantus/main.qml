import QtQuick 2.0
import QtQuick.Controls 1.0
import QtQuick.Controls.Styles 1.0
import QtQuick.Layouts 1.0

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("Cantus - Rustified Pithos for Pandora")

    menuBar: MenuBar {
        Menu {
            title: qsTr("Cantus")
            MenuItem {
                text: qsTr("Settings")
                onTriggered: console.log("Open Settings Modal");
            }
            MenuItem {
                text: qsTr("About")
                onTriggered: console.log("Open About Modal");
            }
            MenuItem {
                text: qsTr("Quit")
                onTriggered: Qt.quit();
            }
        }
    }

    toolBar: ToolBar {
        RowLayout {
            anchors.fill: parent
            Button {
                iconSource: "play.png"
                onClicked: sound_manager.toggle_play();
            }
            Button {
                iconSource: "skip.png"
                onClicked: sound_manager.next_song();
            }
            Slider {
                id: volSlider
                implicitWidth: 100
                value: 0.8
                onValueChanged: sound_manager.volume_change(value);
            }
            Item { Layout.fillWidth: true }
            Button {
                iconSource: "info.png"
            }
            ComboBox {
                model: stationListModel
                Layout.alignment: Qt.AlignRight
                implicitWidth: 200
            }
        }
    }

    Page1{}

    statusBar: StatusBar {
        RowLayout {
            anchors.fill: parent
            Label { text: "status information can go here" }
        }
    }

}
