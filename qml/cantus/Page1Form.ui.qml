import QtQuick 2.0
import QtQuick.Controls 1.0
import QtQuick.Layouts 1.0

Item {
    ListView {
        id: listView
        x: 8
        y: 8
        width: 624
        height: 464
        delegate: Item {
            x: 5
            width: 80
            height: 40
            Row {
                id: row1
                Rectangle {
                    width: 40
                    height: 40
                    color: colorCode
                }

                Text {
                    text: name
                    font.bold: true
                    anchors.verticalCenter: parent.verticalCenter
                }
                spacing: 10
            }
        }
        model: ListModel {
            ListElement {
                name: "Song #1"
                colorCode: "green"
            }
            ListElement {
                name: "Song #2"
                colorCode: "blue"
            }
        }
    }
}
