import QtQuick 2.0
import QtQuick.Controls 2.12

RadioButton {
    property bool paintDisabled: true
    id: radio
    opacity: enabled ? 1 : 0.5

    indicator: Rectangle {
        implicitWidth: 16
        implicitHeight: 16
        x: radio.leftPadding
        y: radio.height / 2 - height / 2
        radius: 4
        border.color: radio.down ? palette.highlight.darker() : palette.base
        color: palette.base

        Rectangle {
            width: 8
            height: 8
            x: 4
            y: 4
            radius: 4
            color: radio.down ? palette.highlight.darker() : palette.highlight
            visible: radio.checked && (radio.paintDisabled || radio.enabled)
        }
    }
}
