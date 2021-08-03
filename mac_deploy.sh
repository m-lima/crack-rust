#!/usr/bin/env bash

while [ -n "${1}" ]
do
  case ${1} in
    --force-bundle) force=1 ;;
    --no-final-deploy) noFinalDeploy=1 ;;
  esac
  shift
done

BASE_DIR=`cargo metadata --format-version 1 | jq -e -r '.workspace_root'`
if [ $? -ne 0 ]
then
  echo -e '[31mWorkspace dir not detected[m'
  exit -1
fi

BUNDLE=`cargo metadata --format-version 1 | jq -e -r '.packages[] | select(.metadata.bundle.name) | .metadata.bundle.name'`
if [ $? -ne 0 ]
then
  echo -e '[31mBundle info not detected[m'
  exit -1
else
  BUNDLE="${BUNDLE}.app"
fi

BINARY=`cargo metadata --format-version 1 | jq -e -r '.packages[].targets[] | select(.kind[] | contains("bin")) | select(.src_path | startswith("'"${BASE_DIR}"'")) | .name'`
if [ $? -ne 0 ]
then
  echo -e '[31mBinary name not detected[m'
  exit -1
fi

QT_LIBS=`qmake -query QT_INSTALL_LIBS`
if [ $? -ne 0 ]
then
  echo -e '[31mCould not find QT libraries[m'
  exit -1
fi

if [ ${force} ] || [ ! -d "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}" ]
then
  cargo bundle --release
else
  echo Skipping build
fi

if [ ! -d "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents/Frameworks" ]
then
  echo "Running macdeployqt ${BASE_DIR}/target/release/bundle/osx/${BUNDLE} -qmldir=${BASE_DIR}/qml"
  macdeployqt "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}" -qmldir="${BASE_DIR}/qml"
else
  echo Skipping deploy
fi

function remap_or_copy {
  local pending=0

  echo "${2}Inspecting ${1}" | sed 's~'"${BASE_DIR}"'~.~'
  for dep in `otool -L $1 | awk '/^[[:blank:]]*@rpath\/Qt/ { sub(/^[[:blank:]]*/, ""); print $1 }'`
  do
    echo "${2}-Analyzing ${dep}" | sed 's~'"${BASE_DIR}"'~.~'
    parsed_dep=`echo "${dep}" | awk '{ sub(/@rpath/, "'"${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents/Frameworks"'"); print }'`
    if [ -f "${parsed_dep}" ]
    then
      target_dep=`echo ${dep} | sed 's~@rpath~@executable_path/../Frameworks~'`
      echo "${2}-Updating with ${target_dep}" | sed 's~'"${BASE_DIR}"'~.~'
      install_name_tool -change "${dep}" "${target_dep}" "${1}"

      remap_or_copy "${parsed_dep}" "${2}-|-"
      pending=$(( pending + $? ))
    else
      target_dep="${QT_LIBS}/"`echo "${dep}" | cut -d'/' -f2`
      if [ -d "${target_dep}" ]
      then
        echo "${2}-Copying ${target_dep} to bundle"
        cp -r "${target_dep}" "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents/Frameworks/."
        pending=$(( pending + 1 ))

        target_dep=`basename "${target_dep}"`
        remap_or_copy "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents/Frameworks/${target_dep}/Versions/A/${target_dep%.framework}" "${2}-|-"
        pending=$(( pending + $? ))
      else
        echo "${2}-[31Library not found ${target_dep}[m"
      fi
    fi
  done

  return $pending
}

function main_loop {
  local pending=0

  echo "Probing ${1}" | sed 's~'"${BASE_DIR}"'~.~'

  for lib in `otool -L "${1}" | awk '/^[[:blank:]]*@executable_path/ { sub(/^[[:blank:]]*@executable_path\/\.\./, "'"${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents"'"); print $1 }'`
  do
    remap_or_copy "${lib}" "|-"
    pending=$(( pending + $? ))
  done

  return $pending
}

while true
do
  main_loop "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}/Contents/MacOS/${BINARY}"
  if (( $? < 1 ))
  then
    break
  fi
done

if [ -z $noFinalDeploy ]
then
  echo "Running final macdeployqt ${BASE_DIR}/target/release/bundle/osx/${BUNDLE} -qmldir=${BASE_DIR}/qml"
  macdeployqt "${BASE_DIR}/target/release/bundle/osx/${BUNDLE}" -qmldir="${BASE_DIR}/qml"
else
  echo Skipping final deploy
fi
