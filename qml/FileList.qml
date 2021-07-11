import QtQuick
import QtQuick.Controls
import Qt.labs.platform

Rectangle {
  id: root

  property url actionIcon
  property color actionColor
  property alias model: list.model

  signal action(int index)

  implicitHeight: list.model.count > 0 ? list.contentHeight + 12 : 0
  radius: 2
  clip: true
  color: palette.base
  border.color: palette.base

  TapHandler {
    onTapped: root.focus = true
  }

  ListView {
    id: list

    readonly property string home: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0].toString().substr(7)

    anchors {
      fill: parent
      topMargin: 6
      bottomMargin: 6
      leftMargin: 10
      rightMargin: 10
    }

    model: ListModel {
    }

    displaced: Transition {
      NumberAnimation {
        property: 'y'
        duration: 200
      }

    }

    delegate: Item {
      property real gonePaint: 0

      width: root.width - 20
      height: 16

      Text {
        text: path.startsWith(list.home) ? path.replace(list.home, '~') : path
        elide: Text.ElideLeft
        color: palette.text

        anchors {
          left: parent.left
          right: action.left
          rightMargin: 10
        }

      }

      IconLabel {
        id: action

        width: 16
        height: 16
        anchors.right: parent.right
        visible: hover.hovered
        icon.color: actionColor
        icon.source: actionIcon

        HoverHandler {
          cursorShape: Qt.PointingHandCursor
        }

        TapHandler {
          onTapped: root.action(index)
        }

      }

      HoverHandler {
        id: hover
      }

    }

  }

}
