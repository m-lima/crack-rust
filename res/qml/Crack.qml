import QtQuick
import QtQuick.Controls
import Cracker

Column {

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  function crack() {
    console.log('Cracking')

    let files = []
    for (let i = 0; i < input.files.count; i++) {
      files.push(input.files.get(i).path)
    }

    cracker.crack(
      parameters.prefix,
      parameters.length,
      parameters.saltCustom,
      parameters.saltValue,
      parameters.useSha256,
      parameters.deviceAutomatic,
      parameters.useGpu,
      input.hashes,
      files
    )
  }

  Cracker {
    id: cracker

    onFound: count.value++
    onProgressed: (progress) => percentage.value = progress
  }

  SpinBox {
    id: count

    value: 0
  }

  ProgressBar {
    id: percentage

    from: 0
    to: 100
    value: 0
  }
}
