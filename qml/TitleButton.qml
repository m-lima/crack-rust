import QtQuick
import QtQuick.Controls

Button {
  property bool active: false
  property color activeColor: palette.highlight

  width: parent.width
  height: 36
  font.bold: true
  font.pointSize: 14
  palette.buttonText: parent.palette.text

  background: Rectangle {
    id: background

    anchors.fill: parent
    color: palette.button
    state: parent.active ? 'Active' : parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

    HoverHandler {
      cursorShape: Qt.PointingHandCursor
    }

    states: [
      State {
        name: 'Active'

        PropertyChanges {
          target: background
          color: activeColor
        }

      },
      State {
        name: 'Hovered'

        PropertyChanges {
          target: background
          color: Qt.tint(palette.button, Color.transparent(palette.highlight, 0.25))
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
        from: 'Active'

        ColorAnimation {
          duration: 200
          property: 'color'
        }

      },
      Transition {
        to: 'Active'

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

      },
      Transition {
        to: 'Down'

        ColorAnimation {
          duration: 200
          property: 'color'
        }

      }
    ]
  }

}
