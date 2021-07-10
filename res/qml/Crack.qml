import QtQuick
import QtQuick.Controls
import Cracker

// TODO: This is a WIP
// TODO: Keep already cracked values visible
// TODO: Have different states for the main button: [Start, Stop, Done]
// TODO: Allow clearing results
Item {
  anchors.fill: parent

  Cracker {
    id: cracker

    onFound: (input, output) => {
      totalProgress.visible = true
      // TODO: Use Set and only increment based on set size
      totalProgress.cracked++;
      totalProgress.requestPaint();
      results.model.append({
        "hash": input.toString(),
        "plain": output.toString()
      });
    }
    onProgressed: (progress) => button.update(progress)
    onError: (error) => message.text = error
  }

  ListView {
    id: results

    anchors {
      fill: parent
      topMargin: 6
      bottomMargin: 6
      leftMargin: 10
      rightMargin: 10
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

  Progress {
    id: button

    anchors.centerIn: parent
    width: Math.min(parent.height, parent.width / 4)
    height: Math.min(parent.height, parent.width / 4)
    onClicked: {
      let files = [];
      for (let i = 0; i < input.files.count; i++) {
        files.push(input.files.get(i).path);
      }
      totalProgress.total = cracker.crack(parameters.prefix, parameters.length, parameters.saltCustom, parameters.saltValue, parameters.useSha256, parameters.deviceAutomatic, parameters.useGpu, input.hashes, files);
    }
  }

  Canvas {
    id: totalProgress

    property int total
    property int cracked

    visible: false
    height: 3
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
      ctx.beginPath();
      ctx.strokeStyle = palette.highlight;
      ctx.lineCap = 'round';
      ctx.lineWidth = 3;
      ctx.moveTo(1, 1);
      ctx.lineTo((width - 1) * cracked / total, 1);
      ctx.stroke();
    }

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
      margins: 10
    }

  }

  Rectangle {
    height: message.implicitHeight + 20
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
