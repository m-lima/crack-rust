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
        if (results.model.get(i).hash == input)
          return ;

      }
      progress.progress++;
      results.model.append({
        "hash": input.toString(),
        "plain": output.toString(),
        "selection": 0
      });
    }
    onProgressed: (progress) => root.progressed(progress)
    onError: (error) => message.text = error
    onRunningChanged: (running) => root.runningChanged(running)
  }

  Shortcut {
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

  TextField {
    id: filter

    placeholderText: qsTr('Filter')

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
      bottom: parent.bottom
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

    // TODO: Handle files (prompt for opening? prompt for saving?)
    // TODO: Allow ergonomic copy
    // TODO: Allow searching
    ListView {
      id: results

      clip: true

      anchors {
        fill: parent
        topMargin: 6
        bottomMargin: 6
      }

      model: ListModel {
      }

      delegate: Column {
        width: parent.width
        visible: hash.includes(filter.text) || plain.includes(filter.text)
        height: visible ? implicitHeight : 0

        Rectangle {
          width: parent.width
          height: textHash.implicitHeight
          color: selection & 1 ? palette.highlight.darker() : 'transparent'

          TapHandler {
            onTapped: selection ^= 1
          }

          Text {
            id: textHash

            color: palette.text
            elide: Text.ElideMiddle
            text: hash

            anchors {
              left: parent.left
              right: parent.right
              leftMargin: 10
              rightMargin: 10
            }

          }

        }

        Rectangle {
          width: parent.width
          height: textPlain.implicitHeight
          color: selection & 2 ? palette.highlight.darker() : 'transparent'

          TapHandler {
            onTapped: selection ^= 2
          }

          Text {
            id: textPlain

            color: palette.highlight
            horizontalAlignment: Text.AlignRight
            text: plain

            anchors {
              left: parent.left
              right: parent.right
              leftMargin: 10
              rightMargin: 10
            }

          }

        }

      }

    }

  }

}
