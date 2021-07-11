import QtQuick
import QtQuick.Controls

Canvas {
  property alias text: filter.text

  height: filter.height
  onPaint: {
    let ctx = getContext('2d');
    if (filter.activeFocus) {
      ctx.strokeStyle = palette.highlight;
      ctx.moveTo(0, height - 1);
      ctx.lineTo(width, height - 1);
      ctx.stroke();
    } else {
      ctx.clearRect(0, height - 2, width, 2);
    }
  }

  IconLabel {
    id: icon

    x: 4
    y: 4
    width: 16
    height: 16
    icon.source: 'qrc:/img/search.svg'
    icon.color: palette.buttonText

    HoverHandler {
      cursorShape: Qt.PointingHandCursor
    }

    TapHandler {
      onTapped: filter.forceActiveFocus()
    }

  }

  TextField {
    id: filter

    placeholderText: qsTr('Filter')
    background.visible: false
    onActiveFocusChanged: parent.requestPaint()
    onAccepted: filter.focus = false

    anchors {
      left: icon.right
      right: parent.right
      leftMargin: 6
    }

    validator: RegularExpressionValidator {
      regularExpression: /[a-fA-F0-9]*/
    }

  }

}
