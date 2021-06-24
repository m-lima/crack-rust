import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Fusion
import QtQuick.Window

ApplicationWindow {
    id: root
    title: 'Hasher'
    visible: true

    width: 400
    height: 400
    x: Screen.width / 2 - 200
    y: Screen.height / 2 - 200

    palette.window: '#353535'
    palette.windowText: '#cccccc'
    palette.base: '#252525'
    palette.alternateBase: '#353535'
    palette.text: '#cccccc'
    palette.button: '#353535'
    palette.buttonText: '#aaaaaa'
    palette.highlight: 'green'
    palette.highlightedText: '#cccccc'

    SwipeView {
        id: content

        anchors {
            top: parent.top
            bottom: footer.top
            left: parent.left
            right: parent.right
        }

        currentIndex: 0
        interactive: false

        Parameters {}

        BigButton {
            text: 'Yooooo'
        }
    }

    Navigation {
        id: footer

        anchors {
            bottom: parent.bottom
            left: parent.left
            right: parent.right
        }

        height: 50
        content: content
    }
}
