import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Fusion
import QtQuick.Window

ApplicationWindow {
  property color colorA: '#9a14cc'
  property color colorB: '#5e0680'
  property color colorCenter: '#008000'
  property color colorD: '#cc2f14'
  property color colorE: '#801806'

  id: root
  title: 'Hasher'
  visible: true

  width: 400
  height: 400
  x: Screen.width / 2 - 200
  y: Screen.height / 2 - 200

  palette.window: '#353535'
  palette.windowText: '#cccccc'
  palette.base: '#252525'
  palette.alternateBase: '#353535'
  palette.text: '#cccccc'
  palette.button: '#353535'
  palette.buttonText: '#aaaaaa'
  palette.highlight: colorCenter
  palette.highlightedText: '#cccccc'

  Item {
    id: content

    anchors {
      top: parent.top
      bottom: footer.top
      left: parent.left
      right: parent.right
    }

    state: 'Parameters'

    states: [
      State {
        name: 'Parameters'
      },
      State {
        name: 'Input'

        PropertyChanges {
          target: parameters
          opacity: 0
        }

        PropertyChanges {
          target: input
          opacity: 1
        }
      }
    ]

    Parameters {
      id: parameters

      visible: opacity > 0

      Behavior on opacity {
        NumberAnimation {
          duration: 200
        }
      }
    }

    Input {
      id: input

      opacity: 0
      visible: opacity > 0

      Behavior on opacity {
        NumberAnimation {
          duration: 200
        }
      }
    }
  }

  Navigation {
    id: footer

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
    }

    height: 50
    content: content
  }
}
