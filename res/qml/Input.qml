import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform

// TODO: The manual layout here is a mess.. Can it be done better?
Column {
  id: root

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  spacing: 10

  TitleButton {
    id: hashesButton

    text: qsTr('Hashes')
    onClicked: hashes.edit()
  }

  Rectangle {
    id: hashes

    x: 20
    width: parent.width - 40
    height: Math.min(hashesEdit.visible ? hashesEdit.implicitHeight : hashesList.height > 0 ? hashesList.height + 12 : 0, root.parent.height - hashesButton.height - 10 - files.height)

    radius: 2
    clip: true
    color: hashesEdit.palette.base
    border.color: hashesEdit.activeFocus ? palette.highlight : palette.base
    state: 'Display'

    states: [
      State {
        name: 'Display'

        PropertyChanges {
          target: hashesList
          visible: true
          focus: true
        }

        PropertyChanges {
          target: hashesScroll
          visible: false
          focus: false
        }
      },
      State {
        name: 'Edit'

        PropertyChanges {
          target: hashesList
          visible: false
          focus: false
        }

        PropertyChanges {
          target: hashesScroll
          visible: true
          focus: true
        }
      }
    ]

    function edit() {
      state = 'Edit'
      hashesEdit.forceActiveFocus()
    }

    ListView {
      id: hashesList

      anchors {
        fill: parent
        topMargin: 6
        bottomMargin: 6
        leftMargin: 10
        rightMargin: 10
      }

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

    Flickable {
      id: hashesScroll

      anchors.fill: parent

      flickableDirection: Flickable.VerticalFlick

      // TODO: Render the detected hashes (maybe use QSyntaxHighlighter)
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
  }

  DropArea {
    id: files

    width: parent.width
    height: filesButton.height + 10 + filesBorder.height

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

    TitleButton {
      id: filesButton

      onClicked: filesDialog.open()
      text: qsTr('Files')
      active: files.containsDrag
    }

    // TODO: Focus not being passed to listView
    Rectangle {
      id: filesBorder

      anchors {
        top: filesButton.bottom
        left: parent.left
        right: parent.right
        topMargin: 10
        leftMargin: 20
        rightMargin: 20
      }

      height: filesList.model.count > 0 ? Math.min(root.parent.height / 4, filesList.contentHeight + 12) : 0

      radius: 2
      clip: true
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
          topMargin: 6
          bottomMargin: 6
          leftMargin: 10
          rightMargin: 10
        }

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
