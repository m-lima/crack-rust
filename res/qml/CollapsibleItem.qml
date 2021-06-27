import QtQuick
import QtQuick.Controls

Column {
  default property alias _children: child.data
  property string title
  property bool expanded: false
  property bool showLine: true
  property int innerSpacing: 5
  signal clicked

  id: root

  width: parent.width

  TitleButton {
    text: root.title
    onClicked: root.clicked()
    active: expanded
    activeColor: palette.window.lighter()
  }

  Pane {
    width: parent.width
    height: root.expanded ? implicitHeight : 0

    clip: height < implicitHeight
    padding: 20

    Column {
      id: child
      width: parent.width
      spacing: root.innerSpacing
    }

    Behavior on height {
      NumberAnimation {
        duration: 200
      }
    }
  }

  Rectangle {
    width: parent.width
    height: 1

    color: palette.window.lighter()
    visible: expanded && showLine
  }
}
