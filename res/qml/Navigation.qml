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
      right: root.backButton === Navigation.BackButton.Full ? parent.right : undefined
    }

    // TODO: When FULL, if window gets resized, there will be a lag
    visible: width > 0
    width: switch (root.backButton) {
      case Navigation.BackButton.None: return 0
      case Navigation.BackButton.Small: return parent.height
      case Navigation.BackButton.Full: undefined
    }

    onClicked: root.back()

    text: root.backButton < Navigation.BackButton.Full ? '' : root.backText
    icon.source: root.backButton < Navigation.BackButton.Full ? 'qrc:/img/left.svg' : ''
    icon.color: palette.buttonText
    palette.button: root.palette.button.lighter(1.3)
    font.bold: true

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

    palette.button: 'green'
    palette.buttonText: '#252525'
    font.bold: true
  }
}
