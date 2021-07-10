mod cracker;
mod extractor;
mod syntaxhighlighter;
mod template;

qmetaobject::qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "App.qml",
    "BigButton.qml",
    "CollapsibleItem.qml",
    "Crack.qml",
    "CrackButton.qml",
    "Input.qml",
    "Main.qml",
    "Navigation.qml",
    "Parameters.qml",
    "Radio.qml",
    "SlidingView.qml",
    "TitleButton.qml",
});

qmetaobject::qrc!(img, "res/img" as "/img" {
    "cancel.svg",
    "cog.svg",
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
    engine.load_file("qrc:/App.qml".into());
    engine.exec();
}
