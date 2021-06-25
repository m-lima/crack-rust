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
        border.color: edit.activeFocus ? palette.highlight : palette.base.darker()

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
