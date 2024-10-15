#pragma once
#include "cxx-qt-lib/qvariant.h"
#include <QtCore/QUuid>

#define impl_qset(T)                                                                      \
  using QSet_##T = QSet<T>;                                                               \
  inline void clear(QSet_##T &set) { set.clear(); }                                       \
  inline bool contains(const QSet_##T &set, const T &item) { return set.contains(item); } \
  inline bool remove(QSet_##T &set, const T &item) { return set.remove(item); }

impl_qset(QUuid);
