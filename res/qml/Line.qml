import QtQuick 2.0
import QtQuick.Controls 2.12

Rectangle {
    property bool horizontal: true

    width: horizontal ? parent.width : 1
    height: horizontal ? 1 : parent.width
}
