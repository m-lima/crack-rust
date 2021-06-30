import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Column {
  property alias prefix: prefix.text
  property alias length: length.value
  property alias saltCustom: saltCustom.checked
  property alias saltValue: saltValue.text
  property alias useSha256: algorithmSha256.checked
  property alias deviceAutomatic: deviceAutomatic.checked
  property alias useGpu: gpu.checked

  property Item _current: null

  id: root

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  state: _current ? 'Expanded' : ''

  states: State {
    name: 'Expanded'
    AnchorChanges {
      target: root
      anchors.verticalCenter: undefined
      anchors.top: parent.top
    }
  }

  transitions: Transition {
    AnchorAnimation {
      duration: 200
    }
  }

  CollapsibleItem {
    id: format
    title: qsTr('Format')
    expanded: root._current === this
    onClicked: root._current = this
    innerSpacing: 10

    // TODO: Select "custom" is the fields were edited
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
    expanded: root._current === this
    onClicked: root._current = this

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
    expanded: root._current === this
    onClicked: root._current = this

    Radio {
      id: algorithmSha256
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
    expanded: root._current === this
    onClicked: root._current = this

    Switch {
      id: deviceAutomatic
      text: qsTr('Automatic')
      checked: true
    }

    Radio {
      id: gpu
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
