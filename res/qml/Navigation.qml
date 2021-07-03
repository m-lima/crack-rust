import QtQuick
import QtQuick.Controls

Item {

  enum BackButton {
    None,
    Small,
    Full
  }

  required property string text
  required property string backText
  required property int backButton

  signal next
  signal previous

  id: root

  BigButton {
    id: back

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
    }

    visible: width > 0
    width: switch (root.backButton) {
      case Navigation.BackButton.None: return 0
      case Navigation.BackButton.Small: return parent.height
      case Navigation.BackButton.Full: return parent.width
    }

    onClicked: root.previous()

    text: root.backButton < Navigation.BackButton.Full ? '' : root.backText
    icon.source: root.backButton < Navigation.BackButton.Full ? 'qrc:/img/left.svg' : ''
    icon.color: palette.buttonText
    palette.button: root.palette.button.lighter(1.3)
    font.bold: true
    font.pointSize: 18

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

    visible: width > 0

    onClicked: root.next()

    text: root.text
    palette.button: 'green'
    palette.buttonText: '#252525'
    font.bold: true
    font.pointSize: 18
  }
}
