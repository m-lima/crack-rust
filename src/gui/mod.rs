// TODO: Remove these once `derive(SimpleListItem)` is fixed
use qmetaobject::{
    QByteArray, QMetaType, QObject, QObjectBox, QVariant, QmlEngine, SimpleListItem,
    SimpleListModel,
};

qmetaobject::qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "Main.qml",
    "BigButton.qml",
    "CollapsibleItem.qml",
    "Input.qml",
    "Navigation.qml",
    "Parameters.qml",
    "Radio.qml",
    "TitleButton.qml",
});

qmetaobject::qrc!(img, "res/img" as "/img" {
    "left.svg",
    "trash.svg",
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

#[derive(QObject, Default)]
struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    crack: qmetaobject::qt_method!(
        #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
        fn crack(&self, prefix: String, length: u8) {
            println!("Prefix: {}, Length: {}", prefix, length);
        }
    ),
}

impl qmetaobject::QSingletonInit for Cracker {
    fn init(&mut self) {}
}

pub fn run() {
    let cracker = std::ffi::CString::new("Cracker").unwrap();

    qml();
    img();

    let templates = QObjectBox::new(
        include!("../../hidden/template.in")
            .iter()
            .collect::<SimpleListModel<_>>(),
    );

    let mut engine = QmlEngine::new();
    qmetaobject::qml_register_singleton_type::<Cracker>(&cracker, 1, 0, &cracker);
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
