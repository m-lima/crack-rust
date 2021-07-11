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
    ctx.fillStyle = palette.base;
    ctx.fillRect(0, 0, width, height);
    // Progress line
    if (progress > 0) {
      ctx.beginPath();
      ctx.fillStyle = root.highlight;
      ctx.fillRect(0, 0, (width - 1) * progress / total, height);
    }
  }

  Connections {
    function onProgressChanged() {
      root.requestPaint();
    }

  }

}
