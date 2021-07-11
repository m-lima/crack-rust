import QtQuick

Item {
  required property int index
  required property int page

  x: page > index ? -parent.width : 0
  width: parent.width
  height: parent.height
  opacity: page === index ? 1 : 0
  visible: opacity > 0
  focus: visible

  Behavior on opacity {
    NumberAnimation {
      duration: 200
    }

  }

  Behavior on x {
    NumberAnimation {
      duration: 200
    }

  }

}
