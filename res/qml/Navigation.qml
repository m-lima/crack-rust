import QtQuick
import QtQuick.Controls

Item {
  required property Item content

  id: root

  BigButton {
    id: back

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
    }

    visible: width > 0
    width: content.state ? 50 : 0

    icon.source: 'qrc:/img/left.svg'
    icon.color: palette.buttonText
    onClicked: content.state = ''
    palette.button: root.palette.button.lighter(1.3)

    Behavior on width {
      NumberAnimation {
        duration: 200
      }
    }
  }

  BigButton {
    id: next

    anchors {
      top: parent.top
      bottom: parent.bottom
      right: parent.right
      left: back.right
    }

    text: 'Next'
    onClicked: content.state = 'Crack'
    state: content.state

    palette.button: 'green'
    palette.buttonText: '#252525'
    font.bold: true
    font.pointSize: 18

    states: State {
      name: 'Crack'
      PropertyChanges {
        target: next
        onClicked: console.log('Crack')
      }
    }

    transitions: [
      Transition {
        to: ''
        SequentialAnimation {
          NumberAnimation {
            target: next
            duration: 100
            to: 1
            property: 'font.pointSize'
          }
          PropertyAction {
            target: next
            value: 'Next'
            property: 'text'
          }
          NumberAnimation {
            target: next
            duration: 100
            to: 18
            property: 'font.pointSize'
          }
        }
      },
      Transition {
        to: 'Crack'
        SequentialAnimation {
          NumberAnimation {
            target: next
            duration: 100
            to: 1
            property: 'font.pointSize'
          }
          PropertyAction {
            target: next
            value: 'Crack'
            property: 'text'
          }
          NumberAnimation {
            target: next
            duration: 100
            to: 18
            property: 'font.pointSize'
          }
        }
      }
    ]
  }
}
