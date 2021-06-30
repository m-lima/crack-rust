#pragma once

#include <QtQuick/QQuickTextDocument>
#include <QtGui/QSyntaxHighlighter>

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
