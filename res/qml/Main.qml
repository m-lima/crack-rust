import QtQuick
import QtQuick.Controls.Fusion
import QtQuick.Window

Item {
  id: root

  states: [
    State {
      name: 'Input'
      PropertyChanges {
        target: content
        page: 1
      }
      PropertyChanges {
        target: navigation
        enabled: input.hashes.length > 0 || input.files.count > 0
        backButton: Navigation.BackButton.Small
        text: qsTr('Crack')
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

  Item {
    property int page: 0

    id: content

    anchors {
      top: parent.top
      bottom: navigation.top
      left: parent.left
      right: parent.right
    }

    SlidingView {
      index: 0
      page: content.page
      Parameters { id: parameters }
    }

    SlidingView {
      index: 1
      page: content.page
      Input { id: input }
    }

    SlidingView {
      index: 2
      page: content.page
      Crack { id: crack }
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

    text: qsTr('Next')
    backButton: Navigation.BackButton.None
    backText: qsTr('Cancel')
    onNext: root.state = 'Input'
  }
}
