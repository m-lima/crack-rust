import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform

// TODO: Manage sizes. Try to replicate same feeling as Parameters
// TODO: THe manual layout here is a mess.. Can it be done better?
Column {
  readonly property int maxHeight: parent.height - hashesButton.height - 20 - filesButton.height - 20 - 20

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  spacing: 10

  Button {
    id: hashesButton

    width: parent.width
    height: 36

    onClicked: hashes.edit()

    contentItem: Text {
      anchors.fill: parent
      verticalAlignment: Text.AlignVCenter
      horizontalAlignment: Text.AlignHCenter
      text: qsTr('Hashes')
      font.bold: true
      font.pointSize: 14
      color: palette.text
    }

    background: Rectangle {
      id: hashesButtonBackground

      anchors.fill: parent
      color: palette.button
      state: parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

      states: [
        State {
          name: 'Hovered'
          PropertyChanges {
            target: hashesButtonBackground
            color: hoverColor()
          }
        },
        State {
          name: 'Down'
          PropertyChanges {
            target: hashesButtonBackground
            color: palette.highlight
          }
        }
      ]

      transitions: [
        Transition {
          from: 'Down'
          ColorAnimation {
            duration: 200
            property: 'color'
          }
        },
        Transition {
          to: 'Down'
          ColorAnimation {
            duration: 200
            property: 'color'
          }
        }
      ]

      function hoverColor() {
        return Qt.rgba(
          (palette.window.r * 3 + palette.highlight.r) / 4,
          (palette.window.g * 3 + palette.highlight.g) / 4,
          (palette.window.b * 3 + palette.highlight.b) / 4,
          1)
        }

        MouseArea {
          anchors.fill: parent
          hoverEnabled: true
          cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
        }
      }
    }

    // TODO: Render the detected hashes (maybe use QSyntaxHighlighter)
    Rectangle {
      id: hashes

      x: 20
      width: parent.width - 40
      height: Math.min(hashesEdit.visible ? hashesEdit.implicitHeight : hashesList.contentHeight, maxHeight - filesList.height)

      radius: 2
      color: hashesEdit.palette.base
      border.color: hashesEdit.activeFocus ? palette.highlight : palette.base

      states: State {
        name: 'Display'

        PropertyChanges {
          target: hashesScroll
          visible: false
          focus: false
        }

        PropertyChanges {
          target: hashesList
          visible: true
          focus: true
        }
      }

      function edit() {
        hashes.state = ''
        hashesEdit.forceActiveFocus()
      }

      Flickable {
        id: hashesScroll

        anchors.fill: parent

        flickableDirection: Flickable.VerticalFlick

        TextArea.flickable: TextArea {
          id: hashesEdit

          wrapMode: TextArea.Wrap
          selectByMouse: true
          placeholderText: qsTr('Enter text from which to extract hashes')

          function handleEnter(evt) {
            if (evt.modifiers & Qt.ShiftModifier) {
              remove(selectionStart, selectionEnd)
              insert(selectionStart, '\n')
              evt.accepted = true
            } else if (evt.modifiers === 0) {
              hashes.state = 'Display'
              evt.accepted = true
            }
          }

          onEditingFinished: hashes.state = 'Display'

          Keys.onReturnPressed: (evt) => handleEnter(evt)
          Keys.onEnterPressed: (evt) => handleEnter(evt)
        }
      }

      ListView {
        id: hashesList

        anchors {
          fill: parent
          topMargin: 10
          bottomMargin: 10
          leftMargin: 10
          rightMargin: 10
        }

        clip: true
        visible: false
        focus: false

        delegate: Text {
          text: modelData
          color: palette.text
        }

        model: [...new Set(hashesEdit.text.match(/([a-fA-F0-9]{16})/g))]

        MouseArea {
          anchors.fill: parent
          onClicked: hashes.edit()
        }
      }
    }

  DropArea {
    id: files

    width: parent.width
    height: filesButton.height + 10 + filesBorder.height + 10

    keys: [ 'text/uri-list' ]

    function toURL(url) {
      return new URL(url)
    }

    function localFile(url) {
      return url.protocol === 'file' && !url.pathname.endsWith('/')
    }

    function unique(url) {
      for (let i = 0; i < filesList.model.count; i++) {
        if (filesList.model.get(i).path === url.pathname) {
          return false
        }
      }
      return true
    }

    function add(urls) {
      let count = filesList.model.count
      urls.map(toURL).filter(localFile).filter(unique).forEach(u => filesList.model.append({ path: u.pathname }))
      return filesList.model.count > count
    }

    onEntered: (evt) => evt.accepted = evt.urls.map(toURL).filter(localFile).length > 0

    onDropped: (evt) => add(evt.urls) && evt.accept()

    FileDialog {
      id: filesDialog

      fileMode: FileDialog.OpenFiles
      folder: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]

      onAccepted: files.add(filesDialog.files)
    }

    Column {
      anchors.fill: parent

      spacing: 10

      Button {
        id: filesButton

        width: parent.width
        height: 36

        onClicked: filesDialog.open()

        contentItem: Text {
          anchors.fill: parent
          verticalAlignment: Text.AlignVCenter
          horizontalAlignment: Text.AlignHCenter
          text: qsTr('Files')
          font.bold: true
          font.pointSize: 14
          color: palette.text
        }

        // TODO: Avoid repetition from CollapsibleItem
        background: Rectangle {
          id: filesButtonBackground
          anchors.fill: parent
          color: palette.button
          state: files.containsDrag ? 'Dropping' : parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

          states: [
            State {
              name: 'Dropping'
              PropertyChanges {
                target: filesButtonBackground
                color: palette.highlight
              }
            },
            State {
              name: 'Hovered'
              PropertyChanges {
                target: filesButtonBackground
                color: hoverColor()
              }
            },
            State {
              name: 'Down'
              PropertyChanges {
                target: filesButtonBackground
                color: palette.highlight
              }
            }
          ]

          transitions: [
            Transition {
              from: ''
              to: 'Hovered'
              PropertyAction {
                property: 'color'
              }
            },
            Transition {
              ColorAnimation {
                duration: 200
                property: 'color'
              }
            }
          ]

          function hoverColor() {
            return Qt.rgba(
              (palette.window.r * 3 + palette.highlight.r) / 4,
              (palette.window.g * 3 + palette.highlight.g) / 4,
              (palette.window.b * 3 + palette.highlight.b) / 4,
              1)
            }

            MouseArea {
              anchors.fill: parent
              hoverEnabled: true
              cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
            }
          }
        }

        // TODO: Focus not being passed to listView
        Rectangle {
          id: filesBorder

          x: 20
          width: parent.width - 40
          // TODO: There's a magic number here: 36 = 16 + 10 * 2 (minimum size of the Hashes input)
          height: Math.min(filesList.contentHeight, maxHeight - 36)

          radius: 2
          color: palette.base
          border.color: files.containsDrag || filesList.activeFocus ? palette.highlight : palette.base

          Behavior on height {
            NumberAnimation {
              duration: 200
            }
          }

          ListView {
            id: filesList

            anchors {
              fill: parent
              leftMargin: 10
              rightMargin: 10
            }

            clip: true
            focus: true

            model: ListModel {}

            // TODO: This NEEDS to render better
            delegate: RowLayout {
              width: filesBorder.width - 20

              Text {
                id: filesPath

                Layout.maximumWidth: parent.width - 16

                text: path
                color: palette.text
              }

              Button {
                visible: filesHover.hovered
                background: Item {}
                icon.source: 'qrc:/img/trash.svg'
                icon.color: colorA
                padding: 0

                onClicked: filesList.model.remove(index)
              }

              HoverHandler {
                id: filesHover
              }
            }
          }
        }
      }
    }
  }
