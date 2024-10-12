#include "cxx-qt-extensions/qvariant.h"

// Need to use a macro here as we can't template because the types
// are always QVariant and bool. So then CXX can't decide which to use.
#define CXX_QT_QVARIANT_CAN_CONVERT_IMPL(typeName, name) \
  bool qvariantCanConvert##name(const QVariant &variant) \
  {                                                      \
    return variant.canConvert<typeName>();               \
  }

namespace rust
{
  namespace cxxqtextensions1
  {
    namespace qvariant
    {
      CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::QUuid, QUuid)
    }
  }
}
