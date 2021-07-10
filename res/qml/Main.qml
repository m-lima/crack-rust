import QtQuick
import QtQuick.Controls.Fusion
import QtQuick.Window

Item {
  id: root

  Item {
    id: content

    property int page: 0

    anchors {
      top: parent.top
      bottom: navigation.top
      left: parent.left
      right: parent.right
    }

    SlidingView {
      index: 0
      page: content.page

      Parameters {
        id: parameters
      }

    }

    SlidingView {
      index: 1
      page: content.page

      Input {
        id: input
      }

    }

    SlidingView {
      index: 2
      page: content.page

      Crack {
        id: crack
      }

    }

  }

  Navigation {
    id: navigation

    height: 50
    text: qsTr('Next')
    backButton: Navigation.BackButton.None
    backText: qsTr('Back')
    onNext: root.state = 'Input'

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
    }

  }

  states: [
    State {
      name: 'Input'

      PropertyChanges {
        target: content
        page: 1
      }

      PropertyChanges {
        target: navigation
        backButton: Navigation.BackButton.Small
        onNext: root.state = 'Crack'
        onBack: root.state = ''
      }

    },
    State {
      name: 'Crack'

      PropertyChanges {
        target: content
        page: 2
      }

      PropertyChanges {
        target: navigation
        backButton: Navigation.BackButton.Full
        onNext: console.log('Crack!')
        onBack: root.state = 'Input'
      }

    }
  ]
}
