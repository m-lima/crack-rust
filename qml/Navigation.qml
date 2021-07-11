import QtQuick

Item {
  id: root

  property alias text: next.caption
  property bool backButton: false
  property bool cancelButton: false

  signal next()
  signal back()

  BigButton {
    id: back

    onClicked: root.back()
    icon.source: 'qrc:/img/left.svg'
    icon.color: palette.buttonText
    palette.button: root.palette.button.lighter(1.3)
    width: root.backButton ? root.height : 0

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
    }

    Behavior on width {
      NumberAnimation {
        duration: 200
      }

    }

  }

  BigButton {
    id: next

    onClicked: root.next()
    palette.button: 'green'
    palette.buttonText: '#252525'

    anchors {
      top: parent.top
      bottom: parent.bottom
      right: cancel.left
      left: back.right
    }

    Behavior on text {
      SequentialAnimation {
        NumberAnimation {
          target: next
          duration: 100
          to: 1
          property: 'font.pointSize'
        }

        PropertyAction {
        }

        NumberAnimation {
          target: next
          duration: 100
          to: 18
          property: 'font.pointSize'
        }

      }

    }

  }

  BigButton {
    id: cancel

    onClicked: console.log('Cancel')
    hoverCaption: qsTr('Cancel')
    palette.button: colorE
    width: root.cancelButton ? root.height * 2 : 0

    anchors {
      top: parent.top
      bottom: parent.bottom
      right: parent.right
    }

    Behavior on width {
      NumberAnimation {
        duration: 200
      }

    }

  }

}
