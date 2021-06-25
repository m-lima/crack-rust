import QtQuick
import QtQuick.Controls

Button {
    id: button

    background: Rectangle {
        property color baseColor: button.palette.button

        id: background

        anchors.fill: parent
        state: button.down ? 'Down' : button.hovered ? 'Hovered' : ''
        gradient: gradient

        Gradient {
            id: gradient

            GradientStop {
                id: gradientTop
                position: 0
                color: background.baseColor.lighter(1.2)
            }

            GradientStop {
                id: gradientBottom
                position: 1
                color: background.baseColor.darker(1.2)
            }
        }

        states: [
            State {
                name: 'Hovered'
                PropertyChanges {
                    target: background
                    baseColor: button.palette.button.darker(1.1)
                }
            },
            State {
                name: 'Down'
                PropertyChanges {
                    target: background
                    baseColor: button.palette.button.lighter(1.1)
                }
            }
        ]

        transitions: [
            Transition {
                to: ''
                ColorAnimation {
                    duration: 200
                    property: 'baseColor'
                }
            },
            Transition {
                from: 'Down'
                to: 'Hovered'
                ColorAnimation {
                    duration: 200
                    property: 'baseColor'
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
