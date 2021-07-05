import QtQuick
import QtQuick.Controls
import Cracker

// TODO: This is a WIP
// TODO: Keep already cracked values visible
// TODO: Have different states for the main button: [Start, Stop, Done]
Item {
  anchors.fill: parent

  Cracker {
    id: cracker

    function start() {
      console.log('Cracking')

      let files = []
      for (let i = 0; i < input.files.count; i++) {
        files.push(input.files.get(i).path)
      }

      cracker.crack(
        parameters.prefix,
        parameters.length,
        parameters.saltCustom,
        parameters.saltValue,
        parameters.useSha256,
        parameters.deviceAutomatic,
        parameters.useGpu,
        input.hashes,
        files
      )
    }

    // onFound: count.value++
    onProgressed: (progress) => button.update(progress)
    onError: (error) => message.text = error
  }

  Progress {
    id: button

    anchors.centerIn: parent
    width: Math.min(parent.height, parent.width / 4)
    height: Math.min(parent.height, parent.width / 4)

    onClicked: cracker.start()
  }

  Rectangle {
    anchors {
      top: parent.top
      left: parent.left
      right: parent.right
    }

    height: message.implicitHeight + 20
    color: app.colorB

    opacity: message.text ? 1: 0
    visible: opacity > 0

    Behavior on opacity {
      NumberAnimation {
        duration: 200
      }
    }

    Text {
      id: message

      anchors {
        fill: parent
        margins: 10
      }

      text: ''
      color: root.palette.buttonText
      font.pointSize: 16
    }

    TapHandler {
      onTapped: message.text = ''
    }
  }
}
