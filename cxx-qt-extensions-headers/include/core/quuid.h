#pragma once

#include <QtCore/QUuid>
#include "rust/cxx.h"

using UuidVariant = QUuid::Variant;
using UuidVersion = QUuid::Version;

namespace rust
{
  namespace cxxqtextensions1
  {
    QUuid quuidNewV3(const QUuid &ns, ::rust::Slice<const ::std::uint8_t> slice);

    QUuid quuidNewV4();

    QUuid quuidNewV5(const QUuid &ns, ::rust::Slice<const ::std::uint8_t> slice);

    QUuid quuidFromString(const QString &string);

    QUuid quuidFromStr(::rust::str string);
  }
}
