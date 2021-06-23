import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Column {
    property Item current: null

    id: root
    y: parent.height / 2 - implicitHeight / 2
    state: current ? 'Expanded' : ''

    states: [
        State {
            name: 'Expanded'
            PropertyChanges {
                target: root
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

    function expand(expanded) {
        current = expanded
    }

    CollapsibleItem {
        id: format
        title: qsTr('Format')
        expanded: root.current === this
        onClicked: root.expand(this)
        innerSpacing: 10

        ComboBox {
            id: templates
            width: parent.width
            textRole: 'name'

            model: if (typeof _templates !== 'undefined') {
                       _templates
                   } else {
                       [
                           { name: 'One', prefix: '1', length: 11 },
                           { name: 'Two', prefix: '2', length: 12 },
                           { name: 'Three', prefix: '3', length: 13 },
                           { name: 'Custom', prefix: '', length: 14 }
                       ]
                   }

            onCurrentIndexChanged: {
                if (Array.isArray(model)) {
                    prefix.text = model[currentIndex].prefix
                    length.value = model[currentIndex].length
                } else {
                    let idx = model.index(currentIndex, 0)
                    prefix.text = model.data(idx, Qt.UserRole + 1)
                    length.value = model.data(idx, Qt.UserRole + 2)
                }
            }
        }

        TextField {
            id: prefix
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
        expanded: root.current === this
        onClicked: root.expand(this)

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
        expanded: root.current === this
        onClicked: root.expand(this)

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
        expanded: root.current === this
        onClicked: root.expand(this)

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
