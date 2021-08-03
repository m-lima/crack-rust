import QtQuick
import QtQuick.Controls.Fusion

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

        current: content.page === 2
        onRunningChanged: (running) => {
          navigation.running = running;
          if (running) {
            progressBar.opacity = 1;
            progressBar.progress = 0;
          } else {
            progressBar.opacity = 0;
          }
        }
        onProgressed: (progress) => progressBar.progress = progress
      }

    }

  }

  ProgressLine {
    id: progressBar

    opacity: 0
    highlight: app.colorA

    anchors {
      bottom: navigation.top
      left: parent.left
      right: parent.right
    }

    Behavior on opacity {
      NumberAnimation {
        duration: 200
      }

    }

  }

  Navigation {
    id: navigation

    property bool running: false

    height: 50
    text: qsTr('Next')
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
        backButton: true
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
        backButton: true
        text: running ? qsTr('Cancel') : qsTr('Crack')
        onNext: running ? crack.stop() : crack.start()
        onBack: root.state = 'Input'
      }

    }
  ]
}
