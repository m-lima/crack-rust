import QtQuick
import QtQuick.Controls
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

  Canvas {
    id: filterBox

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

    anchors {
      top: error.bottom
      left: parent.left
      right: parent.right
      margins: 10
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

    // TODO: Must lose focus
    TextField {
      id: filter

      placeholderText: qsTr('Filter')
      background.visible: false
      onActiveFocusChanged: filterBox.requestPaint()

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

  ProgressLine {
    id: progress

    total: 0

    anchors {
      top: filterBox.bottom
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
    // TODO: Can searching be done better
    ListView {
      id: results

      property int lastSelected: 0

      clip: true

      Shortcut {
        enabled: results.activeFocus
        sequence: StandardKey.SelectAll
        onActivated: {
          for (let i = 0; i < results.model.count; i++) {
            results.model.get(i).selection = 3;
          }
          results.lastSelected = (results.model.count - 1) * 2;
        }
      }

      Shortcut {
        enabled: results.activeFocus
        sequence: StandardKey.Copy
        onActivated: {
          let items = [];
          for (let i = 0; i < results.model.count; i++) {
            let current = results.model.get(i);
            switch (current.selection) {
            case 1:
              items.push(current.hash);
              break;
            case 2:
              items.push(current.plain);
              break;
            case 3:
              items.push(current.hash + ':' + current.plain);
              break;
            }
          }
          clipboard.text = items.join('\n');
          clipboard.selectAll();
          clipboard.copy();
          clipboard.text = '';
        }
      }

      TextEdit {
        id: clipboard

        visible: false
        focus: false
      }

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
            acceptedModifiers: Qt.NoModifier
            onTapped: {
              results.focus = true;
              for (let i = 0; i < results.model.count; i++) {
                results.model.get(i).selection = 0;
              }
              selection = 1;
              if (selection & 1)
                results.lastSelected = index * 2;

            }
          }

          TapHandler {
            acceptedModifiers: Qt.ControlModifier
            onTapped: {
              results.focus = true;
              selection ^= 1;
              if (selection & 1)
                results.lastSelected = index * 2;

            }
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
            acceptedModifiers: Qt.NoModifier
            onTapped: {
              results.focus = true;
              for (let i = 0; i < results.model.count; i++) {
                results.model.get(i).selection = 0;
              }
              selection ^= 2;
              if (selection & 2)
                results.lastSelected = index * 2 + 1;

            }
          }

          TapHandler {
            acceptedModifiers: Qt.ControlModifier
            onTapped: {
              results.focus = true;
              selection ^= 2;
              if (selection & 2)
                results.lastSelected = index * 2 + 1;

            }
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
