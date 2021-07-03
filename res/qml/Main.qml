import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Fusion
import QtQuick.Window

// TODO: Add "hash" flow
ApplicationWindow {
  property color colorA: '#9a14cc'
  property color colorB: '#5e0680'
  property color colorCenter: '#008000'
  property color colorD: '#cc2f14'
  property color colorE: '#801806'

  property int page: 0

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

  function nextState() {
    if (page < 2) {
      page++
    }
  }

  function previousState() {
    if (page > 0) {
      page--
    }
  }

  Item {
    id: content

    anchors {
      top: parent.top
      bottom: navigation.top
      left: parent.left
      right: parent.right
    }

    state: switch (page) {
      case 0: return ''
      case 1: return 'Input'
      case 2: return 'Crack'
    }

    states: [
      State {
        name: 'Input'

        PropertyChanges {
          target: parametersSlider
          opacity: 0
          x: -parent.width
        }

        PropertyChanges {
          target: inputSlider
          opacity: 1
        }

        PropertyChanges {
          target: crack
          opacity: 0
        }

        PropertyChanges {
          target: crack
          opacity: 0
        }

        PropertyChanges {
          target: navigation
          text: qsTr('Crack')
          backButton: Navigation.BackButton.Small
        }
      },
      State {
        name: 'Crack'

        PropertyChanges {
          target: parametersSlider
          opacity: 0
          x: -parent.width
        }

        PropertyChanges {
          target: inputSlider
          opacity: 0
          x: -parent.width
        }

        PropertyChanges {
          target: crack
          opacity: 1
        }

        PropertyChanges {
          target: navigation
          text: ''
          backButton: Navigation.BackButton.Full
        }
      }
    ]

    Item {
      id: parametersSlider

      width: parent.width
      height: parent.height

      visible: opacity > 0
      focus: visible

      Behavior on opacity {
        NumberAnimation {
          duration: 200
        }
      }

      Behavior on x {
        NumberAnimation {
          duration: 200
        }
      }

      Parameters {
        id: parameters
      }
    }

    Item {
      id: inputSlider

      width: parent.width
      height: parent.height

      opacity: 0
      visible: opacity > 0
      focus: visible

      Behavior on opacity {
        NumberAnimation {
          duration: 200
        }
      }

      Behavior on x {
        NumberAnimation {
          duration: 200
        }
      }

      Input {
        id: input
      }
    }

    Crack {
      id: crack

      anchors.fill: parent

      opacity: 0
      visible: opacity > 0
      focus: visible

      Behavior on opacity {
        NumberAnimation {
          duration: 200
        }
      }
    }
  }

  Navigation {
    id: navigation

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
    }

    height: 50

    backButton: Navigation.BackButton.None
    text: qsTr('Next')
    backText: qsTr('Cancel')
    onNext: root.nextState()
    onPrevious: root.previousState()
  }
}
