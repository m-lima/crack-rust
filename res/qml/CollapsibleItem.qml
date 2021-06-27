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
    width: parent.width
    height: 36

    text: root.title
    onClicked: root.clicked()

    contentItem: Text {
      anchors.fill: parent
      verticalAlignment: Text.AlignVCenter
      horizontalAlignment: Text.AlignHCenter
      text: root.title
      font.bold: true
      font.pointSize: 14
      color: palette.text
    }

    background: Rectangle {
      id: background

      anchors.fill: parent

      color: palette.button
      state: root.expanded ? 'Expanded' : parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

      states: [
        State {
          name: 'Expanded'
          PropertyChanges {
            target: background
            color: palette.window.lighter()
          }
        },
        State {
          name: 'Hovered'
          PropertyChanges {
            target: background
            color: hoverColor()
          }
        },
        State {
          name: 'Down'
          PropertyChanges {
            target: background
            color: palette.highlight
          }
        }
      ]

      transitions: [
        Transition {
          to: 'Expanded'
          ColorAnimation {
            duration: 200
            property: 'color'
          }
        },
        Transition {
          from: 'Down'
          ColorAnimation {
            duration: 200
            property: 'color'
          }
        }
      ]

      // TODO: Try Qt.tint() + Color.transparent()
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
      }
    }

    Pane {
      width: parent.width
      height: root.expanded ? implicitHeight : 0

      clip: height < implicitHeight
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

    // TODO: See if we can use Line here
    Rectangle {
      width: parent.width
      height: 1

      color: palette.window.lighter()
      visible: expanded && showLine
    }
  }
