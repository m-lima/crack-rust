import QtQuick
import QtQuick.Controls

Item {
    Rectangle {
        anchors {
            fill: parent
            topMargin: 10
            bottomMargin: 10
            leftMargin: 10
            rightMargin: 10
        }

        radius: 2
        color: edit.palette.base
        border.color: edit.activeFocus ? palette.highlight : palette.text

        Rectangle {
            x: 1;
            y: 1
            width: parent.width - 2
            height: parent.height - 2

            color: "transparent"

            border.color: Color.transparent(palette.highlight, 40 / 255)
            visible: edit.activeFocus
            radius: 1.7
        }

        Flickable {
            anchors.fill: parent
            flickableDirection: Flickable.VerticalFlick

            TextArea.flickable: TextArea {
                id: edit

                wrapMode: TextArea.Wrap
                selectByMouse: true
            }

            ScrollBar.vertical: ScrollBar {}
        }
    }
}
