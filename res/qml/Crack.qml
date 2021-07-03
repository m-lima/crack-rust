import QtQuick
import QtQuick.Controls
import Cracker

Column {

  anchors {
    verticalCenter: parent.verticalCenter
    left: parent.left
    right: parent.right
  }

  Cracker {
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

// // TODO: Focus probably not needed if new screen will be shown
// next.focus = true

// console.log('Crack')

// let files = []
// for (let i = 0; i < input.files.count; i++) {
//   files.push(input.files.get(i).path)
// }

// Cracker.crack(
//   parameters.prefix,
//   parameters.length,
//   parameters.saltCustom,
//   parameters.saltValue,
//   parameters.useSha256,
//   parameters.deviceAutomatic,
//   parameters.useGpu,
//   input.hashes,
//   files
// )
