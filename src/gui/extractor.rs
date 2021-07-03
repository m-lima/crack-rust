use super::syntaxhighlighter::QSyntaxHighlighter;
use qmetaobject::QObject;

use crate::hash;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Extractor {
    base: qmetaobject::qt_base_class!(trait QSyntaxHighlighter),
    useSha256: qmetaobject::qt_property!(bool; NOTIFY onUseSha256Changed),
    onUseSha256Changed: qmetaobject::qt_signal!(),
    color: qmetaobject::qt_property!(qmetaobject::QColor),
    hashes: qmetaobject::qt_method!(fn(&self, text: String) -> qmetaobject::QVariantList),
}

impl QSyntaxHighlighter for Extractor {
    fn highlight_block(&mut self, text: String) {
        self.regex().find_iter(&text).for_each(|m| {
            let start = m.start();
            let count = m.end() - start;
            self.format_text(start, count, self.color)
        });
    }
}

impl Extractor {
    #[inline]
    fn regex(&self) -> &regex::Regex {
        if self.useSha256 {
            <hash::sha256::Hash as hash::Hash>::regex()
        } else {
            <hash::md5::Hash as hash::Hash>::regex()
        }
    }

    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn hashes(&self, text: String) -> qmetaobject::QVariantList {
        let set = self
            .regex()
            .find_iter(&text)
            .map(|m| String::from(m.as_str()))
            .collect::<std::collections::HashSet<_>>();
        set.into_iter().map(qmetaobject::QString::from).collect()
    }
}
