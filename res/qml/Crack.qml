import QtQuick
import QtQuick.Controls
import Cracker

Item {
  anchors.fill: parent

  function crack() {
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

  Cracker {
    id: cracker

    onFound: count.value++
    onProgressed: (progress) => percentage.value = progress
    onError: (error) => message.text = error
  }

  Button {
    id: button

    anchors.centerIn: parent.center
    width: parent.width / 4
    height: parent.width / 4

    background: Rectangle {
      anchors.fill: button
      radius: button.width / 2
      color: 'red'
    }
  }

  // TODO: Make a thin custom progress bar
  ProgressBar {
    id: percentage

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
      margins: 20
    }

    from: 0
    to: 100
    value: 0
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
      font.pointSize: 18
    }
  }
}
