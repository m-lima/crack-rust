import QtQuick
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

    SlidingView {
      index: 0
      page: root.page
      Parameters { id: parameters }
    }

    SlidingView {
      index: 1
      page: root.page
      Input { id: input }
    }

    SlidingView {
      index: 2
      page: root.page
      Crack {}
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

    backButton: switch(root.page) {
      case 0: return Navigation.BackButton.None
      case 1: return Navigation.BackButton.Small
      case 2: return Navigation.BackButton.Full
    }

    text: switch (root.page) {
      case 0: return qsTr('Next')
      case 1: return qsTr('Crack')
      case 2: return ''
    }

    backText: qsTr('Cancel')
    onNext: root.nextState()
    onPrevious: root.previousState()
  }
}
