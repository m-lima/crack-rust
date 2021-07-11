import QtQuick
import QtQuick.Controls

Button {
  id: button

  property int fontSize: 18

  font.pointSize: fontSize

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

  Behavior on text {
    SequentialAnimation {
      NumberAnimation {
        target: contentItem
        duration: 100
        to: 1
        property: 'font.pointSize'
      }

      PropertyAction {
      }

      NumberAnimation {
        target: contentItem
        duration: 100
        to: fontSize
        property: 'font.pointSize'
      }

    }

  }

  Behavior on icon.source {
    SequentialAnimation {
      NumberAnimation {
        target: contentItem
        duration: 100
        to: 0
        property: 'opacity'
      }

      PropertyAction {
      }

      NumberAnimation {
        target: contentItem
        duration: 100
        to: 1
        property: 'opacity'
      }

    }

  }

}