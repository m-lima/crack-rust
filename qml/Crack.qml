import QtQuick
import Qt.labs.platform
import Cracker

Item {
  id: root

  required property bool current

  signal runningChanged(bool running)
  signal progressed(int progress)

  function start() {
    let files = [];
    for (let i = 0; i < input.files.count; i++) {
      files.push(input.files.get(i).path);
    }
    let total = cracker.crack(parameters.prefix, parameters.length, parameters.saltCustom, parameters.saltValue, parameters.useSha256, parameters.deviceAutomatic, parameters.useGpu, parameters.useMask, parameters.customMask, parameters.maskValue, input.hashes, files);
    if (total > 0)
      progress.total = total;

  }

  function stop() {
    cracker.running = false;
  }

  anchors.fill: parent

  Cracker {
    id: cracker

    onFound: (input, output) => {
      for (let i = 0; i < results.model.count; i++) {
        // Implicit conversion for comparison desired
        let current = results.model.get(i);
        if (!current.plain && current.value == input)
          return ;

      }
      progress.progress++;
      results.model.append({
        "value": input.toString(),
        "plain": false,
        "selected": false
      });
      results.model.append({
        "value": output.toString(),
        "plain": true,
        "selected": false
      });
    }
    onProgressed: (progress) => root.progressed(progress)
    onError: (error) => message.text = error
    onRunningChanged: (running) => root.runningChanged(running)
  }

  Shortcut {
    enabled: current
    sequence: StandardKey.Find
    onActivated: filter.forceActiveFocus()
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

  Filter {
    id: filter

    anchors {
      top: error.bottom
      left: parent.left
      right: parent.right
      margins: 10
    }

  }

  ProgressLine {
    id: progress

    total: 0

    anchors {
      top: filter.bottom
      left: parent.left
      right: parent.right
      topMargin: 10
    }

  }

  Rectangle {
    color: palette.base

    anchors {
      top: progress.bottom
      bottom: divider.top
      left: parent.left
      right: parent.right
    }

    Text {
      text: qsTr('No results yet')
      visible: results.model.count < 1
      color: palette.buttonText

      anchors {
        centerIn: parent
      }

    }

    Results {
      id: results

      filter: filter.text
      clip: true

      anchors {
        fill: parent
        topMargin: 6
        bottomMargin: 6
      }

    }

  }

  Rectangle {
    id: divider

    visible: files.visible
    height: 1
    color: palette.window

    anchors {
      bottom: files.top
      left: parent.left
      right: parent.right
    }

  }

  FileList {
    id: files

    property string current

    visible: results.model.count > 0
    height: visible ? Math.min(parent.height / 4, implicitHeight) : 0
    model: input.files
    actionIcon: 'qrc:/img/save.svg'
    actionColor: palette.highlight
    onAction: (index) => {
      files.current = files.model.get(index).path;
      saveDialog.file = 'file://' + files.current + '.cracked';
      saveDialog.open();
    }

    FileDialog {
      id: saveDialog

      fileMode: FileDialog.SaveFile
      onAccepted: {
        let resultList = [];
        for (let i = 0; i < results.model.count; i++) {
          resultList.push(results.model.get(i).value);
        }
        cracker.save(files.current, new URL(saveDialog.file).pathname, resultList);
      }
    }

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
    }

  }

}
