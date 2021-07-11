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

  Button {
    id: filterIcon

    width: height
    icon.source: 'qrc:/img/search.svg'
    icon.color: palette.buttonText
    background.visible: false
    onClicked: filter.forceActiveFocus()

    HoverHandler {
      cursorShape: Qt.PointingHandCursor
    }

  }

  TextField {
    id: filter

    placeholderText: qsTr('Filter')
    background.visible: false
    onActiveFocusChanged: parent.requestPaint()

    anchors {
      left: filterIcon.right
      right: parent.right
      leftMargin: 2
    }

    validator: RegularExpressionValidator {
      regularExpression: /[a-fA-F0-9]*/
    }

  }

}
