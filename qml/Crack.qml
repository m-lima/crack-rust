import QtQuick
import QtQuick.Controls
import Cracker

Item {
  id: root

  signal runningChanged(bool running)
  signal progressed(int progress)

    function start() {
      let files = [];
      for (let i = 0; i < input.files.count; i++) {
        files.push(input.files.get(i).path);
      }
      let total = cracker.crack(parameters.prefix, parameters.length, parameters.saltCustom, parameters.saltValue, parameters.useSha256, parameters.deviceAutomatic, parameters.useGpu, input.hashes, files);
      if (total > 0)
        crackedProgress.total = total;

    }

    function stop() {
      cracker.running = false
    }

  anchors.fill: parent

  Cracker {
    id: cracker

    onFound: (input, output) => {
      for (let i = 0; i < results.model.count; i++) {
        // Implicit conversion for comparison desired
        if (results.model.get(i).hash == input)
          return ;

      }
      crackedProgress.progress++;
      crackedProgress.requestPaint();
      results.model.append({
        "hash": input.toString(),
        "plain": output.toString()
      });
    }
    onProgressed: (progress) => root.progressed(progress)
    onError: (error) => message.text = error
    onRunningChanged: (running) => root.runningChanged(running)
  }

  // TODO: Allow ergonomic copy
  // TODO: Allow searching
  ListView {
    id: results

    clip: true

    anchors {
      top: error.bottom
      bottom: crackedProgress.top
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

  ProgressLine {
    id: crackedProgress

    total: 0

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
