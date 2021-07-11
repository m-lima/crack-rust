import QtQuick

ListView {
  id: root

  property int lastSelected: 0
  required property string filter

  Shortcut {
    enabled: root.activeFocus
    sequence: StandardKey.SelectAll
    onActivated: {
      for (let i = 0; i < root.model.count; i++) {
        root.model.get(i).selected = true;
      }
      root.lastSelected = root.model.count - 1;
    }
  }

  Shortcut {
    enabled: root.activeFocus
    sequence: StandardKey.Copy
    onActivated: {
      let items = [];
      for (let i = 0; i < root.model.count; i++) {
        let current = root.model.get(i);
        if (current.selected) {
          if (current.plain) {
            items.push(current.value);
          } else {
            let next = root.model.get(++i);
            if (next.selected)
              items.push(current.value + ':' + next.value);
            else
              items.push(current.value);
          }
        }
      }
      clipboard.text = items.join('\n');
      clipboard.selectAll();
      clipboard.copy();
      clipboard.text = '';
    }
  }

  TextEdit {
    id: clipboard

    visible: false
    focus: false
  }

  TapHandler {
    onTapped: root.focus = true
  }

  model: ListModel {
  }

  delegate: Rectangle {
    width: parent.width
    visible: value.includes(filter) || (plain ? root.model.get(index - 1).value.includes(filter) : root.model.get(index + 1).value.includes(filter))
    height: visible ? textLabel.implicitHeight : 0
    color: selected ? palette.highlight.darker() : 'transparent'

    TapHandler {
      acceptedModifiers: Qt.NoModifier
      onTapped: {
        for (let i = 0; i < root.model.count; i++) {
          root.model.get(i).selected = false;
        }
        if (selected) {
          selected = false;
        } else {
          selected = true;
          root.lastSelected = index;
        }
      }
    }

    TapHandler {
      acceptedModifiers: Qt.ControlModifier
      onTapped: {
        if (selected) {
          selected = false;
        } else {
          selected = true;
          root.lastSelected = index;
        }
      }
    }

    TapHandler {
      acceptedModifiers: Qt.ShiftModifier
      onTapped: {
        let step = root.lastSelected < index ? -1 : 1;
        for (let i = index; i !== root.lastSelected; i += step) {
          root.model.get(i).selected = true;
        }
      }
    }

    Text {
      id: textLabel

      width: parent.width
      color: plain ? palette.highlight : palette.text
      elide: plain ? Text.ElideNone : Text.ElideMiddle
      horizontalAlignment: plain ? Text.AlignRight : Text.AlignLeft
      text: value

      anchors {
        left: parent.left
        right: parent.right
        leftMargin: 10
        rightMargin: 10
      }

    }

  }

}
