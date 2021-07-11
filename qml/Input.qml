import QtQuick
import QtQuick.Controls
import Qt.labs.platform
import HashExtractor

Column {
  id: root

  property alias hashes: hashesList.model
  property alias files: filesList.model

  spacing: 10

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  TitleButton {
    id: hashesButton

    text: qsTr('Hashes')
    onClicked: hashes.edit()
  }

  Rectangle {
    id: hashes

    function edit() {
      state = 'Edit';
      hashesEdit.forceActiveFocus();
    }

    x: 20
    width: parent.width - 40
    height: 0
    radius: 2
    clip: true
    color: hashesEdit.palette.base
    border.color: hashesEdit.activeFocus ? palette.highlight : palette.base
    state: 'Display'

    ListView {
      id: hashesList

      model: []

      anchors {
        fill: parent
        topMargin: 6
        bottomMargin: 6
        leftMargin: 10
        rightMargin: 10
      }

      TapHandler {
        onTapped: hashes.edit()
      }

      delegate: Text {
        width: parent.width
        text: modelData
        color: palette.text
        elide: Text.ElideMiddle
      }

    }

    // TODO: Text is getting clipped during animation
    Flickable {
      id: hashesScroll

      anchors.fill: parent
      flickableDirection: Flickable.VerticalFlick

      TextArea.flickable: TextArea {
        id: hashesEdit

        function handleEnter(evt) {
          if (evt.modifiers & Qt.ShiftModifier) {
            remove(selectionStart, selectionEnd);
            insert(selectionStart, '\n');
            evt.accepted = true;
          } else if (evt.modifiers === 0) {
            hashes.state = 'Display';
            evt.accepted = true;
          }
        }

        wrapMode: TextArea.Wrap
        selectByMouse: true
        placeholderText: qsTr('Enter text from which to extract hashes')
        palette.text: '#888888'
        Keys.onReturnPressed: (evt) => handleEnter(evt)
        Keys.onEnterPressed: (evt) => handleEnter(evt)
        onEditingFinished: {
          hashesList.model = hashesExtractor.hashes(hashesEdit.text);
          hashes.state = 'Display';
        }

        HashExtractor {
          id: hashesExtractor

          textDocument: hashesEdit.textDocument
          color: root.palette.text
          useSha256: parameters.useSha256
          onUseSha256Changed: {
            this.rehighlight();
            hashesList.model = this.hashes(hashesEdit.text);
          }
        }

      }

    }

    states: [
      State {
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

        PropertyChanges {
          target: hashes
          height: Math.min(hashesList.contentHeight > 0 ? hashesList.contentHeight + 12 : 0, root.parent.height - hashesButton.height - 10 - files.height - 10)
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

        PropertyChanges {
          target: hashes
          height: Math.min(hashesEdit.implicitHeight, root.parent.height - hashesButton.height - 10 - files.height - 10)
        }

      }
    ]

    transitions: Transition {
      NumberAnimation {
        property: 'height'
        duration: 200
      }

    }

  }

  DropArea {
    id: files

    function toURL(url) {
      return new URL(url);
    }

    function localFile(url) {
      return url.protocol === 'file' && !url.pathname.endsWith('/');
    }

    function unique(url) {
      for (let i = 0; i < filesList.model.count; i++) {
        if (filesList.model.get(i).path === url.pathname)
          return false;

      }
      return true;
    }

    function add(urls) {
      let count = filesList.model.count;
      urls.map(toURL).filter(localFile).filter(unique).forEach((u) => filesList.model.append({
        "path": u.pathname
      }));
      return filesList.model.count > count;
    }

    width: parent.width
    height: filesButton.height + 10 + filesList.height + 10
    keys: ['text/uri-list']
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

    FileList {
      id: filesList

      x: 20
      y: filesButton.height + 10
      width: parent.width - 40
      height: Math.min(root.parent.height / 4, filesList.implicitHeight)
      border.color: files.containsDrag ? palette.highlight : palette.base
      actionIcon: 'qrc:/img/trash.svg'

      Behavior on height {
        NumberAnimation {
          duration: 200
        }

      }

    }

  }

}
