import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Fusion
import QtQuick.Layouts
import QtQuick.Window

ApplicationWindow {
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

    Column {
        property Item current: null

        id: content
        y: (parent.height - next.height) / 2 - (implicitHeight / 2)

        width: parent.width

        states: [
            State {
                name: 'Expanded'
                PropertyChanges {
                    target: content
                    y: 0
                }
            }
        ]

        transitions: [
            Transition {
                NumberAnimation {
                    duration: 200
                    property: 'y'
                }
            }
        ]

        NumberAnimation {
            id: moveUpAnimation
            target: content
            property: 'y'
            from: (content.parent.height - next.height) / 2 - (content.implicitHeight / 2)
            to: 0
            duration: 200
        }

        function expand(expanded) {
            if (!state) {
              state = 'Expanded'
            }

            current = expanded
        }

        CollapsibleItem {
            id: format
            title: qsTr('Format')
            expanded: content.current === this
            onClicked: content.expand(this)
            innerSpacing: 10

            ComboBox {
                id: templates
                width: parent.width
                textRole: 'name'

                // TODO: add imported model
                // model: _templates
                model: ListModel{
                    ListElement { name: 'One'; prefix: '1'; length: 11 }
                    ListElement { name: 'Two'; prefix: '2'; length: 12 }
                    ListElement { name: 'Three'; prefix: '3'; length: 13 }
                    ListElement { name: 'Custom'; prefix: ''; length: 14 }
                }

               delegate: MenuItem {
                   width: ListView.view.width
                   text: name
                   font.weight: index == templates.currentIndex ? Font.DemiBold : Font.Normal
                   highlighted: index == templates.highlightedIndex
                   hoverEnabled: true
                   onClicked: {
                       formatPrefix.text = prefix
                       formatLength.value = length
                   }
               }
            }

            TextField {
                id: formatPrefix
                width: parent.width
                placeholderText: qsTr('Prefix')
                maximumLength: 25
                validator: RegularExpressionValidator {
                    regularExpression: /[0-9]{0,25}/
                }
            }

            RowLayout {
                width: parent.width

                Text {
                    text: qsTr('Length:')
                    color: palette.buttonText
                }

                SpinBox {
                    id: formatLength
                    value: 12
                    from: Math.max(formatPrefix.text.length, 3)
                    to: 25
                    Layout.fillWidth: true
                }
            }
        }

        CollapsibleItem {
            // TODO: Add OPET
            title: qsTr('Salt')
            expanded: content.current === this
            onClicked: content.expand(this)

            Switch {
                id: saltCustom
                text: qsTr('Custom')
                checked: false
                onCheckedChanged: saltCustom.checked && saltValue.forceActiveFocus()
            }

            TextField {
                id: saltValue
                width: parent.width
                enabled: saltCustom.checked
                placeholderText: qsTr('Salt')
                opacity: saltCustom.checked ? 1 : 0.5
            }
        }

        CollapsibleItem {
            title: qsTr('Algorithm')
            expanded: content.current === this
            onClicked: content.expand(this)

            Radio {
                text: qsTr('Sha256')
                checked: true
            }
            Radio {
                text: qsTr('Md5')
            }
        }

        CollapsibleItem {
            title: qsTr('Device')
            showLine: false
            expanded: content.current === this
            onClicked: content.expand(this)

            Switch {
                id: deviceAutomatic
                text: qsTr('Automatic')
                checked: true
            }
            Radio {
                text: qsTr('GPU')
                enabled: !deviceAutomatic.checked
                checked: true
                paintDisabled: false
            }
            Radio {
                text: qsTr('CPU')
                enabled: !deviceAutomatic.checked
                paintDisabled: false
            }
        }
    }

    Button {
        id: next
        height: 50
        y: parent.height - height
        width: parent.width

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
            state: next.down ? 'Down' : next.hovered ? 'Hovered' : ''

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



