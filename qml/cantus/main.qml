import QtQuick 2.0
import QtQuick.Controls 1.0
import QtQuick.Layouts 1.0

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("Cantus - Rustified Pithos for Pandora")

    menuBar: MenuBar {
        Menu {
            title: "Cantus"
            MenuItem { text: "Settings" }
            MenuItem { text: "About" }
            MenuItem { text: "Quit" }
        }
    }

    toolBar: ToolBar {
        RowLayout {
            anchors.fill: parent
            CheckBox {
                text: "test box 1"
                checked: false
                Layout.alignment: Qt.AlignRight
            }
            Item { Layout.fillWidth: true }
            CheckBox {
                text: "test box 2"
                checked: true
                Layout.alignment: Qt.AlignRight
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
