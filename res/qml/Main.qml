import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQuick.Layouts 1.15

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
        y: current ? 0 : (parent.height - next.height) / 2 - (implicitHeight / 2)

        width: parent.width

        NumberAnimation {
            id: moveUpAnimation
            target: content
            property: 'y'
            from: (content.parent.height - next.height) / 2 - (content.implicitHeight / 2)
            to: 0
            duration: 200
        }

        function expand(expanded) {
            if (y > 0) {
              moveUpAnimation.start()
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
                width: parent.width
                textRole: 'name'

                model: if (typeof _templates !== 'undefined') {
                           return _templates
                       } else {
                           return [
                             { name: 'Custom', prefix: '', length: 10 },
                             { name: 'One', prefix: '1', length: 11 },
                             { name: 'Two', prefix: '2', length: 12 },
                             { name: 'Three', prefix: '3', length: 13 }
                           ]
                       }

                onCurrentIndexChanged: {
                    if (currentIndex) {
                        console.log('Index:', currentIndex)
                        console.log('Model:', model)
                        console.log('Model.data:', model.data)
                        console.log('Model[]:', model[currentIndex])
                        console.log('Model.data[]:', model.data[currentIndex])
                        prefix.text = model.data(model.index(currentIndex, 0), 0, 'prefix')
                        length.value = model.data(model.index(currentIndex, 0), 0, Qt.UserRole + 2)
                    } else {
                        console.log('Index: ', currentIndex)
                    }
                }
            }

            TextField {
                id: prefix
                width: parent.width
                placeholderText: qsTr('Prefix')
                maximumLength: 25
                validator: RegExpValidator {
                    regExp: /[0-9]{0,25}/
                }
            }

            RowLayout {
                width: parent.width

                Text {
                    text: qsTr('Length:')
                    color: palette.buttonText
                }

                SpinBox {
                    id: length
                    value: 12
                    from: Math.max(prefix.text.length, 3)
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
            anchors.fill: parent
            color: next.down ? Qt.lighter(palette.highlight, 1.2) : parent.hovered ? Qt.darker(palette.highlight, 1.2) : palette.highlight

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
                onPressed: mouse.accepted = false
            }

            // TODO: Make hover start instantaneous
            Behavior on color {
                ColorAnimation {
                    duration: 200
                }
            }
        }
    }
}



