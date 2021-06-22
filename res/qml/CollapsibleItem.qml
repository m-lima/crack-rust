import QtQuick
import QtQuick.Controls

Column {
    default property alias _children: child.data
    property string title
    property bool expanded: false
    property bool showLine: true
    property int innerSpacing: 5
    signal clicked

    id: root

    width: parent.width

    Button {
        text: root.title
        onClicked: root.clicked()
        height: 36
        width: parent.width

        contentItem: Text {
            anchors.fill: parent
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignHCenter
            text: parent.text
            font.bold: true
            font.pointSize: 14
            color: palette.text
        }

        background: Rectangle {
            anchors.fill: parent
            color: root.expanded ? palette.window.lighter() : parent.hovered ? hoverColor() : parent.down ? palette.highlight : palette.button

            function hoverColor() {
                return Qt.rgba(
                            (palette.window.r * 3 + palette.highlight.r) / 4,
                            (palette.window.g * 3 + palette.highlight.g) / 4,
                            (palette.window.b * 3 + palette.highlight.b) / 4,
                            1)
            }

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
            }

            // TODO: Make hover start instantaneous
            Behavior on color {
                ColorAnimation {
                    duration: 200
                }
            }
        }
    }

    Pane {
        clip: height < implicitHeight
        height: parent.expanded ? implicitHeight : 0
        width: parent.width
        padding: 20

        Column {
            id: child
            width: parent.width
            spacing: root.innerSpacing
        }

        Behavior on height {
            NumberAnimation {
                duration: 200
            }
        }
    }

    Rectangle {
        width: parent.width
        height: 1
        color: palette.window.lighter()
        visible: expanded && showLine
    }
}
