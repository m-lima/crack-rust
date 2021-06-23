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

    // TODO: Transition is wrong
    StackView {
        id: stack

        anchors {
            top: parent.top
            bottom: footer.top
            right: parent.right
            left: parent.left
        }

        initialItem: parameters

        Parameters {
            id: parameters
        }

        Loader {
            id: two
            active: stack.depth > 1
            Button {
              text: 'Yooooo'
            }
        }
    }

    Item {
        id: footer

        anchors.bottom: parent.bottom
        width: parent.width
        height: 50

        Button {
            id: back

            anchors {
                top: parent.top
                bottom: parent.bottom
                left: parent.left
            }

            visible: width > 0
            width: stack.depth > 1 ? 50 : 0

            icon.source: 'qrc:/img/left.svg'
            icon.color: palette.buttonText
            onClicked: stack.pop()

            Behavior on width {
                NumberAnimation {
                    duration: 200
                    easing.type: Easing.Linear
                }
            }
        }

        Button {
            id: next

            anchors {
                top: parent.top
                bottom: parent.bottom
                right: parent.right
                left: back.right
            }

            text: 'Next'
            onClicked: stack.push(two)

            palette.button: 'darkgreen'
            palette.buttonText: '#252525'
            font.bold: true
            font.pointSize: 18

//            contentItem: Text {
//                anchors.fill: parent
//                verticalAlignment: Text.AlignVCenter
//                horizontalAlignment: Text.AlignHCenter
//                text: qsTr('Next')
//                font.bold: true
//                font.pointSize: 18
//                color: palette.base
//            }

//            background: Rectangle {
//                id: background

//                anchors.fill: parent
//                color: palette.highlight
//                border.color: color.lighter(1.2)
//                border.width: 1
//                state: parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

//                states: [
//                    State {
//                        name: 'Hovered'
//                        PropertyChanges {
//                            target: background
//                            color: palette.highlight.darker(1.2)
//                        }
//                    },
//                    State {
//                        name: 'Down'
//                        PropertyChanges {
//                            target: background
//                            color: palette.highlight.lighter(1.2)
//                        }
//                    }
//                ]

//                transitions: [
//                    Transition {
//                        to: ''
//                        ColorAnimation {
//                            duration: 200
//                            property: 'color'
//                        }
//                    },
//                    Transition {
//                        from: 'Down'
//                        to: 'Hovered'
//                        ColorAnimation {
//                            duration: 200
//                            property: 'color'
//                        }
//                    }
//                ]

//                MouseArea {
//                    anchors.fill: parent
//                    hoverEnabled: true
//                    cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
//                }
//            }
        }
    }
}
