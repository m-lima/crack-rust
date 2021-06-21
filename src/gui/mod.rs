qmetaobject::qrc!(qml, "res/qml" as "/" {
    "Main.qml",
    "CollapsibleItem.qml",
    "Radio.qml",
});

pub fn run() {
    qml();
    let mut engine = qmetaobject::QmlEngine::new();
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
