#pragma once
#include "cxx-qt-lib/qlist.h"
#include <QtCore/QUuid>

#define impl_qlist(T)                                  \
  using QList_##T = QList<T>;                          \
  inline void clear(QList_##T &list) { list.clear(); } \
  inline bool contains(const QList_##T &list, const T &item) { return list.contains(item); }

impl_qlist(QUuid);
