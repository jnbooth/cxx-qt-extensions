#pragma once
#include "cxx-qt-lib/qvector.h"
#include <QtCore/QUuid>

#define impl_qvector(T)                                \
  using QVector_##T = QVector<T>;                      \
  inline void clear(QVector_##T &vec) { vec.clear(); } \
  inline bool contains(const QVector_##T &vec, const T &item) { return vec.contains(item); }

impl_qvector(QUuid);
