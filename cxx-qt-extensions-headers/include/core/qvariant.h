#include "cxx-qt-lib/qvariant.h"
#include <QtCore/QUuid>

namespace rust
{
  namespace cxxqtextensions1
  {
    namespace qvariant
    {
      using rust::cxxqtlib1::qvariant::qvariantConstruct;
      using rust::cxxqtlib1::qvariant::qvariantValueOrDefault;

      CXX_QT_QVARIANT_CAN_CONVERT(QUuid)
    }
  }
}
