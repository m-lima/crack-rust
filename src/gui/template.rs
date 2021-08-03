#[derive(qmetaobject::SimpleListItem, Clone, Default)]
struct Template {
    pub name: String,
    pub prefix: String,
    pub length: u8,
    pub mask: bool,
}

impl Template {
    fn new(name: &str, prefix: &str, length: u8, mask: bool) -> Self {
        Self {
            name: String::from(name),
            prefix: String::from(prefix),
            length,
            mask,
        }
    }
}

pub fn build() -> qmetaobject::SimpleListModel<impl qmetaobject::SimpleListItem> {
    include!("../../hidden/template.in").iter().collect()
}
