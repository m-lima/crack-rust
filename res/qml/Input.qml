import QtQuick
import QtQuick.Controls

Item {
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
    border.color: edit.activeFocus ? palette.highlight : palette.base.darker()

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
      leftMargin: 10
      rightMargin: 10
    }

    height: 56
    keys: [ 'text/uri-list' ]

    Rectangle {
      anchors.fill: parent

      radius: 2
      color: palette.base
      border.color: parent.containsDrag ? colorA: activeFocus ? palette.highlight : palette.base
    }

    ListView {
      id: fileList

      anchors {
        fill: parent
        leftMargin: 10
        rightMargin: 10
      }

      clip: true

      model: ListModel {}

      delegate: Text {
        text: path
        color: palette.text
      }
    }

    function localFile(url) {
      let parsedUrl = new URL(url)
      return parsedUrl.protocol === 'file' && !parsedUrl.pathname.endsWith('/')
    }

    onEntered: (evt) => evt.accepted = evt.urls.filter(localFile).length > 0

    onDropped: (evt) => evt.urls.filter(localFile).forEach(u => fileList.model.append({ path: u.toString().substr(7) }))
  }
}
