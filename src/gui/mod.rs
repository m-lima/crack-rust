use qmetaobject::*;

qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "Main.qml",
    "BigButton.qml",
    "CollapsibleItem.qml",
    "Input.qml",
    "Navigation.qml",
    "Parameters.qml",
    "Radio.qml",
});

qrc!(img, "res/img" as "/img" {
    "left.svg",
});

#[derive(SimpleListItem, Clone, Default)]
struct Template {
    pub name: String,
    pub prefix: String,
    pub length: u8,
}

impl Template {
    fn new(name: &str, prefix: &str, length: u8) -> Self {
        Self {
            name: String::from(name),
            prefix: String::from(prefix),
            length,
        }
    }
}

pub fn run() {
    qml();
    img();

    let templates = QObjectBox::new(
        include!("../../hidden/template.in")
            .iter()
            .collect::<SimpleListModel<_>>(),
    );

    let mut engine = QmlEngine::new();
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
