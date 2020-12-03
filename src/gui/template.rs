pub const TEMPLATES: &[Template] = include!("../../hidden/template.in");

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Template {
    name: &'static str,
    prefix: &'static str,
    length: std::os::raw::c_int,
}

impl Template {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn prefix(&self) -> &'static str {
        self.prefix
    }

    pub const fn length(&self) -> std::os::raw::c_int {
        self.length
    }
}

pub unsafe fn to_q_list() -> qt_widgets::cpp_core::CppBox<qt_widgets::qt_core::QStringList> {
    TEMPLATES.iter().map(Template::name).fold(
        qt_widgets::qt_core::QStringList::new(),
        |acc, curr| {
            acc.append_q_string(&qt_widgets::qt_core::qs(curr));
            acc
        },
    )
}
