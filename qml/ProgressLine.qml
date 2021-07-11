import QtQuick

Canvas {
  id: root

  property int total: 100
  property int progress: 0
  property color highlight: palette.highlight

  implicitHeight: 3
  onPaint: {
    let ctx = getContext('2d');
    // Base line
    ctx.beginPath();
    ctx.strokeStyle = palette.base;
    ctx.lineCap = 'round';
    ctx.lineWidth = 3;
    ctx.moveTo(1, 1);
    ctx.lineTo(width - 1, 1);
    ctx.stroke();
    // Progress line
    if (progress > 0) {
      ctx.beginPath();
      ctx.strokeStyle = root.highlight;
      ctx.lineCap = 'round';
      ctx.lineWidth = 3;
      ctx.moveTo(1, 1);
      ctx.lineTo((width - 1) * progress / total, 1);
      ctx.stroke();
    }
  }

  Connections {
    function onProgressChanged() {
      root.requestPaint();
    }

  }

}
