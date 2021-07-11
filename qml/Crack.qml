import QtQuick
import QtQuick.Controls
import Cracker

Item {
  required property Item footer
  anchors.fill: parent

  Cracker {
    id: cracker

    onFound: (input, output) => {
      for (let i = 0; i < results.model.count; i++) {
        // Implicit conversion for comparison desired
        if (results.model.get(i).hash == input)
          return ;

      }
      totalProgress.cracked++;
      totalProgress.requestPaint();
      results.model.append({
        "hash": input.toString(),
        "plain": output.toString()
      });
    }
    onProgressed: (progress) => crackButton.progress = progress
    onError: (error) => message.text = error
    onRunningChanged: (running) => {
      if (running) {
        crackButton.state = 'Running';
      } else {
        crackButton.state = '';
        crackButton.progress = 0;
      }
      footer.cancelButton = running
    }
  }

  // TODO: Allow ergonomic copy
  ListView {
    id: results

    clip: true

    anchors {
      top: error.bottom
      bottom: totalProgress.top
      left: parent.left
      right: parent.right
      margins: 10
    }

    model: ListModel {
    }

    delegate: Column {
      width: parent.width

      Text {
        width: parent.width
        color: palette.text
        elide: Text.ElideMiddle
        text: hash
      }

      Text {
        width: parent.width
        color: palette.highlight
        horizontalAlignment: Text.AlignRight
        text: plain
      }

    }

  }

  CrackButton {
    id: crackButton

    caption: 'Crack'
    image: 'qrc:/img/cog.svg'
    hoverColor: root.palette.highlight
    anchors.centerIn: parent
    width: Math.min(parent.height, parent.width) / 2
    height: Math.min(parent.height, parent.width) / 2
    onClicked: {
      let files = [];
      for (let i = 0; i < input.files.count; i++) {
        files.push(input.files.get(i).path);
      }
      let total = cracker.crack(parameters.prefix, parameters.length, parameters.saltCustom, parameters.saltValue, parameters.useSha256, parameters.deviceAutomatic, parameters.useGpu, input.hashes, files);
      if (total > 0)
        totalProgress.total = total;

    }
    states: [
      State {
        name: 'Running'

        PropertyChanges {
          target: crackButton
          caption: ''
          captionHover: 'Stop'
          image: ''
          imageHover: 'qrc:/img/cancel.svg'
          hoverColor: colorD.lighter(1.5)
          onClicked: cracker.running = false
        }

      }
    ]
  }

  Canvas {
    id: totalProgress

    property int total: 0
    property int cracked: 0

    height: total > 0 ? 3 : 0
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
      if (cracked > 0) {
        ctx.beginPath();
        ctx.strokeStyle = palette.highlight;
        ctx.lineCap = 'round';
        ctx.lineWidth = 3;
        ctx.moveTo(1, 1);
        ctx.lineTo((width - 1) * cracked / total, 1);
        ctx.stroke();
      }
    }

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
      margins: 10
    }

  }

  Rectangle {
    id: error

    height: message.text ? message.implicitHeight + 20 : 0
    color: app.colorB
    opacity: message.text ? 1 : 0
    visible: opacity > 0

    anchors {
      top: parent.top
      left: parent.left
      right: parent.right
    }

    Text {
      id: message

      text: ''
      color: root.palette.buttonText
      font.pointSize: 16

      anchors {
        fill: parent
        margins: 10
      }

    }

    TapHandler {
      onTapped: message.text = ''
    }

    Behavior on opacity {
      NumberAnimation {
        duration: 200
      }

    }

  }

}
