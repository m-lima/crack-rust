mod cracker;
mod extractor;
mod syntaxhighlighter;
mod template;

qmetaobject::qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "BigButton.qml",
    "CollapsibleItem.qml",
    "Crack.qml",
    "Input.qml",
    "Main.qml",
    "Navigation.qml",
    "Parameters.qml",
    "Radio.qml",
    "SlidingView.qml",
    "TitleButton.qml",
});

qmetaobject::qrc!(img, "res/img" as "/img" {
    "left.svg",
    "trash.svg",
});

pub fn run() {
    let cracker = std::ffi::CString::new("Cracker").unwrap();
    let extractor = std::ffi::CString::new("HashExtractor").unwrap();

    qml();
    img();

    let templates = qmetaobject::QObjectBox::new(template::build());

    let mut engine = qmetaobject::QmlEngine::new();
    qmetaobject::qml_register_type::<cracker::Cracker>(&cracker, 1, 0, &cracker);
    qmetaobject::qml_register_type::<extractor::Extractor>(&extractor, 1, 0, &extractor);

    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
