use qmetaobject::*;

qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "Main.qml",
    "CollapsibleItem.qml",
    "Radio.qml",
});

#[derive(SimpleListItem, Default)]
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

    let templates = QObjectBox::new(
        vec![
            Template::new("Custom", "", 8),
            Template::new("TP", "923", 4),
        ]
        .into_iter()
        .collect::<SimpleListModel<_>>(),
    );

    let mut engine = QmlEngine::new();
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
