import QtQuick
import QtQuick.Controls
import Cracker

Button {
  property int progress: 0

  signal running
  signal done

  id: button

  function update(progress) {
    this.progress = progress
    background.requestPaint()
  }

  // TODO: It's not really "progress" job to be both the trigger and the progress
  states: [
    State {
      name: 'Running'
      PropertyChanges {
        icon.source: ''
        text: progress + '%'
      }
      PropertyChanges {
        target: background
        visible: true
      }
    }
  ]

  palette.buttonText: hover.hovered ? root.palette.highlight : root.palette.buttonText
  icon.source: 'qrc:/img/cog.svg'
  text: 'Crack'
  icon.color: root.palette.buttonText
  icon.width: width / 2
  icon.height: width / 2
  display: AbstractButton.TextUnderIcon
  font.pixelSize: width / 4

  background: Canvas {
    id: background

    visible: false

    anchors.centerIn: parent
    width: Math.min(parent.height, parent.width)
    height: Math.min(parent.height, parent.width)

    onPaint: {
      if (!visible) {
        return;
      }

      let ctx = getContext('2d')

      ctx.reset()

      if (hover.hovered) {
        ctx.beginPath()
        ctx.fillStyle = palette.base
        ctx.lineWidth = 0
        ctx.roundedRect(width  / 4, width / 4, width / 2, width / 2, width / 16, width / 16)
        ctx.fill()
      }

      ctx.beginPath()
      ctx.strokeStyle = palette.base
      ctx.lineWidth = 2
      ctx.ellipse(2, 2, width - 4, width - 4)
      ctx.stroke()

      ctx.beginPath()
      ctx.strokeStyle = palette.highlight
      ctx.lineCap = 'round'
      ctx.lineWidth = 2
      ctx.arc(width / 2, width / 2, width / 2 - 2, Math.PI / 2, Math.PI / 2 + Math.PI * 2 * button.progress / 100)
      ctx.stroke()
    }
  }

  HoverHandler {
    id: hover
    cursorShape: Qt.PointingHandCursor
    onHoveredChanged: background.requestPaint()
  }
}
