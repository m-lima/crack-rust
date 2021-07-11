import QtQuick
import QtQuick.Controls

Button {
  id: button

  font.pointSize: 18
  font.bold: true

  background: Rectangle {
    id: background

    property color baseColor: button.palette.button

    anchors.fill: parent
    state: button.down ? 'Down' : button.hovered ? 'Hovered' : ''
    gradient: gradient

    Gradient {
      id: gradient

      GradientStop {
        position: 0
        color: background.baseColor.lighter(1.2)
      }

      GradientStop {
        position: 1
        color: background.baseColor.darker(1.2)
      }

    }

    HoverHandler {
      cursorShape: Qt.PointingHandCursor
    }

    states: [
      State {
        name: 'Hovered'

        PropertyChanges {
          target: background
          baseColor: button.palette.button.darker(1.1)
        }

      },
      State {
        name: 'Down'

        PropertyChanges {
          target: background
          baseColor: button.palette.button.lighter(1.1)
        }

      }
    ]
    transitions: [
      Transition {
        to: ''

        ColorAnimation {
          duration: 200
          property: 'baseColor'
        }

      },
      Transition {
        from: 'Down'
        to: 'Hovered'

        ColorAnimation {
          duration: 200
          property: 'baseColor'
        }

      }
    ]
  }

}
