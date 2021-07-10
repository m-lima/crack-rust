import QtQuick
import QtQuick.Controls
import Cracker

Button {
  id: button

  property int progress: 0
  property string caption
  property string captionHover
  property string image
  property string imageHover
  property color hoverColor

  palette.buttonText: hover.hovered && hoverColor ? hoverColor : root.palette.buttonText
  text: hover.hovered && captionHover ? captionHover : caption ? caption : progress + '%'
  icon.source: hover.hovered && imageHover ? imageHover : image
  icon.color: palette.buttonText
  icon.width: width / 3
  icon.height: width / 3
  display: AbstractButton.TextUnderIcon
  font.pixelSize: width / 4

  Connections {
    function onProgressChanged() {
      background.requestPaint();
    }
  }

  // TODO: Mask it to match the rounded shape
  HoverHandler {
    id: hover

    cursorShape: Qt.PointingHandCursor
  }

  background: Canvas {
    id: background

    anchors.centerIn: parent
    width: Math.max(4, Math.min(parent.height, parent.width))
    height: width
    onPaint: {
      let ctx = getContext('2d');

      // Base circle
      ctx.beginPath();
      ctx.strokeStyle = palette.base;
      ctx.lineWidth = 2;
      ctx.ellipse(2, 2, width - 4, width - 4);
      ctx.stroke();
      // Progress circle

      if (progress > 0) {
        ctx.beginPath();
        ctx.strokeStyle = palette.highlight;
        ctx.lineCap = 'round';
        ctx.lineWidth = 2;
        ctx.arc(width / 2, width / 2, width / 2 - 2, Math.PI / 2, Math.PI / 2 + Math.PI * 2 * button.progress / 100);
        ctx.stroke();
      }
    }
  }

}
