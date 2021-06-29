use cpp::cpp;

use qmetaobject::*;

/// This trait allow to override a Qt QSyntaxHighlighter
pub trait QSyntaxHighlighter: QObject {
    /// Required for the implementation detail of the QObject custom derive
    fn get_object_description() -> &'static QObjectDescription
    where
        Self: Sized,
    {
        unsafe {
            &*cpp!([]-> *const QObjectDescription as "RustObjectDescription const*" {
                return rustObjectDescription<Rust_QSyntaxHighlighter>();
            })
        }
    }

    /// Refer to the Qt documentation of QSyntaxHighlighter::document
    fn document(&self) -> Option<QQuickTextDocument> {
        let obj = self.get_cpp_object();
        let text_document = cpp!(unsafe [obj as "Rust_QSyntaxHighlighter *"] -> QQuickTextDocument as "const QQuickTextDocument *" {
            return obj ? obj->quickDocument() : QQuickTextDocument();
        });
    }

    /// Refer to the Qt documentation of QSyntaxHighlighter::setDocument
    fn set_document(&mut self, document: QQuickTextDocument) -> QModelIndex {
        let obj = self.get_cpp_object();
        cpp!(unsafe [obj as "Rust_QSyntaxHighlighter *", document as "QQuickTextDocument"] -> QQuickTextDocument as "QQuickTextDocument" {
            return obj ? obj->document() : QQuickTextDocument();
        })
    }
}

cpp! {{
    #include <qmetaobject_rust.hpp>
    #include <QtGui/QSyntaxHighlighter>
}}

cpp! {{
    class Rust_QSyntaxHighlighterProxy : public QSyntaxHighlighter {
        Q_OBJECT

        Q_PROPERTY(QQuickTextDocument *textDocument MEMBER m_TextDocument WRITE setTextDocument NOTIFY textDocumentChanged)

        public:
        Rust_QSyntaxHighlighterProxy(QObject *parent = 0) : QSyntaxHighlighter(parent) {
            m_TextDocument = nullptr;
        }

        signals:
        void textDocumentChanged();

        protected:
        void setTextDocument(QQuickTextDocument *textDocument) {
            if (m_TextDocument == textDocument) {
                return;
            }

            m_TextDocument = textDocument;

            setDocument(m_TextDocument ? m_TextDocument->textDocument() : nullptr);

            emit textDocumentChanged();
        }

        QQuickTextDocument *m_TextDocument;
    }

    struct Rust_QSyntaxHighlighter : RustObject<QSyntaxHighlighter> {

        using QAbstractItemModel::beginInsertRows;
        using QAbstractItemModel::endInsertRows;
        using QAbstractItemModel::beginRemoveRows;
        using QAbstractItemModel::endRemoveRows;
        using QAbstractItemModel::beginResetModel;
        using QAbstractItemModel::endResetModel;
        using QAbstractItemModel::createIndex;
        using QAbstractItemModel::changePersistentIndexList;
        using QAbstractItemModel::persistentIndexList;

        QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override {
            return rust!(Rust_QAbstractItemModel_index [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                row: i32 as "int",
                column: i32 as "int",
                parent: QModelIndex as "QModelIndex"
            ] -> QModelIndex as "QModelIndex" {
                rust_object.borrow().index(row, column, parent)
            });
        }

        QModelIndex parent(const QModelIndex &index) const override {
            return rust!(Rust_QAbstractItemModel_parent [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                index : QModelIndex as "QModelIndex"
            ] -> QModelIndex as "QModelIndex" {
                rust_object.borrow().parent(index)
            });
        }

        int rowCount(const QModelIndex &parent = QModelIndex()) const override {
            return rust!(Rust_QAbstractItemModel_rowCount [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                parent: QModelIndex as "QModelIndex"
            ] -> i32 as "int" {
                rust_object.borrow().row_count(parent)
            });
        }

        int columnCount(const QModelIndex &parent = QModelIndex()) const override {
            return rust!(Rust_QAbstractItemModel_columnCount [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                parent : QModelIndex as "QModelIndex"
            ] -> i32 as "int" {
                rust_object.borrow().column_count(parent)
            });
        }

        QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override {
            return rust!(Rust_QAbstractItemModel_data [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                index: QModelIndex as "QModelIndex",
                role: i32 as "int"
            ] -> QVariant as "QVariant" {
                rust_object.borrow().data(index, role)
            });
        }

        bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override {
            return rust!(Rust_QAbstractItemModel_setData [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                index: QModelIndex as "QModelIndex",
                value: QVariant as "QVariant",
                role: i32 as "int"
            ] -> bool as "bool" {
                rust_object.borrow_mut().set_data(index, &value, role)
            });
        }

        QHash<int, QByteArray> roleNames() const override {
            QHash<int, QByteArray> base = QAbstractItemModel::roleNames();
            rust!(Rust_QAbstractItemModel_roleNames [
                rust_object: QObjectPinned<dyn QAbstractItemModel> as "TraitObject",
                base: *mut c_void as "QHash<int, QByteArray> &"
            ] {
                for (key, val) in rust_object.borrow().role_names().iter() {
                    add_to_hash(base, *key, val.clone());
                }
            });
            return base;
        }
    };
}}
