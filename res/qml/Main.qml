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

    Item {
        id: content

        width: root.width
        height: root.height - footer.height

        // TODO: Transition is wrong
        StackView {
            id: stack

            anchors.fill: parent

            initialItem: parameters

            Parameters {
                id: parameters
            }

            Parameters {
                id: two
            }
        }
    }

    // TODO: Should we have these items that dictate the layout?
    Item {
        id: footer

        y: root.height - height
        width: root.width
        height: 50

        Item {
            anchors.fill: parent

            Button {
                id: back

                visible: width > 0
                width: stack.depth > 1 ? implicitWidth : 0
                height: parent.height

                text: 'Back'
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

                x: back.width
                width: parent.width - back.width
                height: parent.height

                onClicked: stack.push(two)

                contentItem: Text {
                    anchors.fill: parent
                    verticalAlignment: Text.AlignVCenter
                    horizontalAlignment: Text.AlignHCenter
                    text: qsTr('Next')
                    font.bold: true
                    font.pointSize: 18
                    color: palette.base
                }

                background: Rectangle {
                    id: background

                    anchors.fill: parent
                    color: palette.highlight
                    border.color: color.lighter(1.2)
                    border.width: 1
                    state: parent.down ? 'Down' : parent.hovered ? 'Hovered' : ''

                    states: [
                        State {
                            name: 'Hovered'
                            PropertyChanges {
                                target: background
                                color: palette.highlight.darker(1.2)
                            }
                        },
                        State {
                            name: 'Down'
                            PropertyChanges {
                                target: background
                                color: palette.highlight.lighter(1.2)
                            }
                        }
                    ]

                    transitions: [
                        Transition {
                            to: ''
                            ColorAnimation {
                                duration: 200
                                property: 'color'
                            }
                        },
                        Transition {
                            from: 'Down'
                            to: 'Hovered'
                            ColorAnimation {
                                duration: 200
                                property: 'color'
                            }
                        }
                    ]

                    MouseArea {
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
                    }
                }
            }
        }
    }
}
