import QtQuick

Item {

  enum BackButton {
    None,
    Small,
    Full
  }

  property alias text: next.text
  required property string backText
  required property int backButton

  signal next
  signal back

  id: root

  BigButton {
    id: back

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
    }

    state: switch(root.backButton) {
      case Navigation.BackButton.None: return ''
      case Navigation.BackButton.Small: return 'Small'
      case Navigation.BackButton.Full: return 'Full'
    }

    width: 0

    onClicked: root.back()

    icon.source: 'qrc:/img/left.svg'
    icon.color: palette.buttonText
    palette.button: root.palette.button.lighter(1.3)
    font.bold: true

    states: [
      State {
        name: 'Small'
        PropertyChanges {
          target: back
          width: root.height
        }
      },
      State {
        name: 'Full'
        PropertyChanges {
          target: back
          width: undefined
        }
        AnchorChanges {
          target: back
          anchors.right: root.right
        }
      }
    ]

    transitions: [
      Transition {
        NumberAnimation {
          duration: 200
          property: 'width'
        }
        AnchorAnimation {
          duration: 200
        }
      }
    ]
  }

  BigButton {
    id: next

    anchors {
      top: parent.top
      bottom: parent.bottom
      right: parent.right
      left: back.right
    }

    onClicked: root.next()

    palette.button: 'green'
    palette.buttonText: '#252525'
    font.bold: true
  }
}
