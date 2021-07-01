/****************************************************************************
** Meta object code from reading C++ file 'QSyntaxHighlighterProxy.hpp'
**
** Created by: The Qt Meta Object Compiler version 68 (Qt 6.1.1)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "QSyntaxHighlighterProxy.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'QSyntaxHighlighterProxy.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 68
#error "This file was generated using the moc from 6.1.1. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_QSyntaxHighlighterProxy_t {
    const uint offsetsAndSize[10];
    char stringdata0[78];
};
#define QT_MOC_LITERAL(ofs, len) \
    uint(offsetof(qt_meta_stringdata_QSyntaxHighlighterProxy_t, stringdata0) + ofs), len 
static const qt_meta_stringdata_QSyntaxHighlighterProxy_t qt_meta_stringdata_QSyntaxHighlighterProxy = {
    {
QT_MOC_LITERAL(0, 23), // "QSyntaxHighlighterProxy"
QT_MOC_LITERAL(24, 19), // "textDocumentChanged"
QT_MOC_LITERAL(44, 0), // ""
QT_MOC_LITERAL(45, 12), // "textDocument"
QT_MOC_LITERAL(58, 19) // "QQuickTextDocument*"

    },
    "QSyntaxHighlighterProxy\0textDocumentChanged\0"
    "\0textDocument\0QQuickTextDocument*"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_QSyntaxHighlighterProxy[] = {

 // content:
       9,       // revision
       0,       // classname
       0,    0, // classinfo
       1,   14, // methods
       1,   21, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags, initial metatype offsets
       1,    0,   20,    2, 0x06,    1 /* Public */,

 // signals: parameters
    QMetaType::Void,

 // properties: name, type, flags
       3, 0x80000000 | 4, 0x0001510b, uint(0), 0,

       0        // eod
};

void QSyntaxHighlighterProxy::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<QSyntaxHighlighterProxy *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->textDocumentChanged(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (QSyntaxHighlighterProxy::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QSyntaxHighlighterProxy::textDocumentChanged)) {
                *result = 0;
                return;
            }
        }
    }
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty) {
        auto *_t = static_cast<QSyntaxHighlighterProxy *>(_o);
        (void)_t;
        void *_v = _a[0];
        switch (_id) {
        case 0: *reinterpret_cast< QQuickTextDocument**>(_v) = _t->textDocument(); break;
        default: break;
        }
    } else if (_c == QMetaObject::WriteProperty) {
        auto *_t = static_cast<QSyntaxHighlighterProxy *>(_o);
        (void)_t;
        void *_v = _a[0];
        switch (_id) {
        case 0: _t->setTextDocument(*reinterpret_cast< QQuickTextDocument**>(_v)); break;
        default: break;
        }
    } else if (_c == QMetaObject::ResetProperty) {
    } else if (_c == QMetaObject::BindableProperty) {
    }
#endif // QT_NO_PROPERTIES
    (void)_a;
}

const QMetaObject QSyntaxHighlighterProxy::staticMetaObject = { {
    QMetaObject::SuperData::link<QSyntaxHighlighter::staticMetaObject>(),
    qt_meta_stringdata_QSyntaxHighlighterProxy.offsetsAndSize,
    qt_meta_data_QSyntaxHighlighterProxy,
    qt_static_metacall,
    nullptr,
qt_incomplete_metaTypeArray<qt_meta_stringdata_QSyntaxHighlighterProxy_t
, QtPrivate::TypeAndForceComplete<QQuickTextDocument*, std::true_type>, QtPrivate::TypeAndForceComplete<void, std::false_type>



>,
    nullptr
} };


const QMetaObject *QSyntaxHighlighterProxy::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *QSyntaxHighlighterProxy::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_QSyntaxHighlighterProxy.stringdata0))
        return static_cast<void*>(this);
    return QSyntaxHighlighter::qt_metacast(_clname);
}

int QSyntaxHighlighterProxy::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QSyntaxHighlighter::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 1)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 1;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 1)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 1;
    }
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty || _c == QMetaObject::WriteProperty
            || _c == QMetaObject::ResetProperty || _c == QMetaObject::BindableProperty
            || _c == QMetaObject::RegisterPropertyMetaType) {
        qt_static_metacall(this, _c, _id, _a);
        _id -= 1;
    }
#endif // QT_NO_PROPERTIES
    return _id;
}

// SIGNAL 0
void QSyntaxHighlighterProxy::textDocumentChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
