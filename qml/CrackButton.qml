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

  signal clicked()

  palette.buttonText: hover.hovered && background.hovered && hoverColor ? hoverColor : root.palette.buttonText
  text: hover.hovered && background.hovered && captionHover ? captionHover : caption ? caption : progress + '%'
  icon.source: hover.hovered && background.hovered && imageHover ? imageHover : image
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

  TapHandler {
    onTapped: background.hovered && button.clicked()
  }

  background: Canvas {
    id: background

    property bool hovered: {
      let radius = background.width / 2;
      let x = radius - hover.point.position.x;
      let y = radius - hover.point.position.y;
      let distance = x * x + y * y;
      radius *= radius;
      distance < radius;
    }

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

    HoverHandler {
      id: hover

      target: button.contentItem
      cursorShape: background.hovered ? Qt.PointingHandCursor : undefined
    }

  }

}
