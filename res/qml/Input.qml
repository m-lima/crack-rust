import QtQuick
import QtQuick.Controls
import Qt.labs.platform

// TODO: Add section titles
Item {
  anchors.fill: parent

  // TODO: Render the detected hashes
  Rectangle {
    anchors {
      top: parent.top
      bottom: files.top
      left: parent.left
      right: parent.right
      topMargin: 10
      bottomMargin: 10
      leftMargin: 10
      rightMargin: 10
    }

    radius: 2
    color: edit.palette.base
    border.color: edit.activeFocus ? palette.highlight : palette.base

    Flickable {
      anchors.fill: parent
      flickableDirection: Flickable.VerticalFlick

      TextArea.flickable: TextArea {
        id: edit

        wrapMode: TextArea.Wrap
        selectByMouse: true
      }

      ScrollBar.vertical: ScrollBar {}
    }
  }

  DropArea {
    id: files

    anchors {
      bottom: parent.bottom
      left: parent.left
      right: parent.right
      bottomMargin: 10
    }

    height: fileColumn.implicitHeight

    keys: [ 'text/uri-list' ]

    function toURL(url) {
      return new URL(url)
    }

    function localFile(url) {
      return url.protocol === 'file' && !url.pathname.endsWith('/')
    }

    function unique(url) {
      for (let i = 0; i < fileList.model.count; i++) {
        if (fileList.model.get(i).path === url.pathname) {
          return false
        }
      }
      return true
    }

    function add(urls) {
      let count = fileList.model.count
      urls.map(toURL).filter(localFile).filter(unique).forEach(u => fileList.model.append({ path: u.pathname }))
      return fileList.model.count > count
    }

    onEntered: (evt) => evt.accepted = evt.urls.map(toURL).filter(localFile).length > 0

    onDropped: (evt) => add(evt.urls) && evt.accept()

    FileDialog {
      id: fileDialog

      fileMode: FileDialog.OpenFiles
      folder: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]

      onAccepted: files.add(fileDialog.files)
    }

    Column {
      id: fileColumn

      width: parent.width

      spacing: 10

      Button {
        id: fileButton

        width: parent.width
        height: 36

        onClicked: fileDialog.open()

        contentItem: Text {
          anchors.fill: parent
          verticalAlignment: Text.AlignVCenter
          horizontalAlignment: Text.AlignHCenter
          text: 'Files'
          font.bold: true
          font.pointSize: 14
          color: palette.text
        }

        // TODO: Avoid repetition from CollapsibleItem
        background: Rectangle {
          id: background
          anchors.fill: parent
          color: palette.button
          state: files.containsDrag ? 'Dropping' : parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

          states: [
            State {
              name: 'Dropping'
              PropertyChanges {
                target: background
                color: palette.highlight
              }
            },
            State {
              name: 'Hovered'
              PropertyChanges {
                target: background
                color: hoverColor()
              }
            },
            State {
              name: 'Down'
              PropertyChanges {
                target: background
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

        Rectangle {
          x: 10
          width: parent.width - 20
          height: fileList.model.count > 0 ? 56 : 0

          radius: 2
          color: palette.base
          border.color: files.containsDrag || fileList.activeFocus ? palette.highlight : palette.base

          Behavior on height {
            NumberAnimation {
              duration: 200
            }
          }

          ListView {
            id: fileList

            anchors {
              fill: parent
              leftMargin: 10
              rightMargin: 10
            }

            clip: true
            focus: true

            model: ListModel {}

            // TODO: Add delete button
            delegate: Text {
              text: path
              color: palette.text
            }

            highlight: Rectangle {
              color: palette.highlight
            }
          }
        }
      }
    }
  }
