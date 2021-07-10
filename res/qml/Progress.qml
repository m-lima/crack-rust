import QtQuick
import QtQuick.Controls
import Cracker

Button {
  id: button

  property int progress: 0

  signal cancel()

  function update(progress) {
    this.progress = progress;
    background.requestPaint();
  }

  palette.buttonText: hover.hovered ? colorD.lighter(1.5) : root.palette.buttonText
  text: hover.hovered ? 'Stop' : progress + '%'
  icon.source: hover.hovered ? 'qrc:/img/cog.svg' : ''
  icon.color: palette.buttonText
  icon.width: width / 3
  icon.height: width / 3
  display: AbstractButton.TextUnderIcon
  font.pixelSize: width / 4

  // TODO: Mask it to match the rounded shape
  HoverHandler {
    id: hover

    cursorShape: Qt.PointingHandCursor
  }

  background: Canvas {
    id: background

    anchors.centerIn: parent
    width: Math.min(parent.height, parent.width)
    height: Math.min(parent.height, parent.width)
    onPaint: {
      let ctx = getContext('2d');

      // Base circle
      ctx.beginPath();
      ctx.strokeStyle = palette.base;
      ctx.lineWidth = 2;
      ctx.ellipse(2, 2, width - 4, width - 4);
      ctx.stroke();

      // Progress circle
      ctx.beginPath();
      ctx.strokeStyle = palette.highlight;
      ctx.lineCap = 'round';
      ctx.lineWidth = 2;
      ctx.arc(width / 2, width / 2, width / 2 - 2, Math.PI / 2, Math.PI / 2 + Math.PI * 2 * button.progress / 100);
      ctx.stroke();
    }
  }

}
