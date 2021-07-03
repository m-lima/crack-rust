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
    width: content.state === 'Input' ? 50 : 0

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

    text: qsTr('Next')
    onClicked: content.state = 'Input'
    state: content.state

    palette.button: 'green'
    palette.buttonText: '#252525'
    font.bold: true
    font.pointSize: 18

    states: [
      State {
        name: 'Input'
        PropertyChanges {
          target: next
          onClicked: {
            content.state = 'Crack'
            crack.crack()
          }
        }
      },
      State {
        name: 'Crack'
        PropertyChanges {
          target: next
          palette.button: colorD
          onClicked: content.state = ''
        }
      }
    ]

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
            value: qsTr('Next')
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
        to: 'Input'
        SequentialAnimation {
          NumberAnimation {
            target: next
            duration: 100
            to: 1
            property: 'font.pointSize'
          }
          PropertyAction {
            target: next
            value: qsTr('Crack')
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
      // TODO: Rethink presentation of the cancel button
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
            value: qsTr('Cancel')
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
