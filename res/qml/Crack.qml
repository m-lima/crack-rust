import QtQuick
import QtQuick.Controls
import Cracker

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

  Button {
    property int progress: 0

    id: button

    function update(progress) {
      this.progress = progress
      background.requestPaint()
    }

    anchors.centerIn: parent
    width: parent.width / 4
    height: parent.width / 4

    text: progress + '%'
    font.pixelSize: width / 4

    background: Canvas {
      id: background

      anchors.fill: parent
      onPaint: {
        let ctx = getContext('2d')

        ctx.beginPath()
        ctx.strokeStyle = palette.base
        ctx.lineWidth = 2
        ctx.ellipse(2, 2, width - 4, width - 4)
        ctx.stroke()

        ctx.beginPath()
        ctx.arc(width / 2, width / 2, width / 2 - 2, Math.PI / 2, Math.PI / 2 + Math.PI * 2 * button.progress / 100)
        ctx.strokeStyle = palette.highlight
        ctx.lineCap = 'round'
        ctx.lineWidth = 2
        ctx.stroke()
      }
    }

    onClicked: cracker.start()
  }

  // TODO: Make a thin custom progress bar
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
