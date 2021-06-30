#pragma once

#include <iostream>
#include <QtQuick/QQuickTextDocument>
#include <QtGui/QSyntaxHighlighter>

// This class needs to be passed into QT's MOC. The output of which needs to be copied as
// QSyntaxHighlighterProxy.cpp in this folder.  Minor fixes are needed to adjust the paths
//
// If calling QT's MOC is not straightforward, a QT project with just this hpp can be created
// and the results of the build process copied over
class QSyntaxHighlighterProxy : public QSyntaxHighlighter {
  Q_OBJECT
  Q_PROPERTY(QQuickTextDocument *textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

  public:
  explicit QSyntaxHighlighterProxy(QObject *parent = nullptr) : QSyntaxHighlighter(parent) {
    m_TextDocument = nullptr;
  }

  QQuickTextDocument *textDocument() const {
    return m_TextDocument;
  }

  void setTextDocument(QQuickTextDocument *textDocument) {
    if (m_TextDocument == textDocument) {
      return;
    }

    m_TextDocument = textDocument;

    setDocument(m_TextDocument ? m_TextDocument->textDocument() : nullptr);

    emit textDocumentChanged();
  }

  void setColorFormat(int start, int length, QColor color) {
    setFormat(start, length, color);
  }

  signals:
  void textDocumentChanged();

  private:
  QQuickTextDocument *m_TextDocument;
};
