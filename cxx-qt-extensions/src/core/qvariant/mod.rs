use crate::QUuid;
use cxx_qt_lib::{QVariant, QVariantValue};

macro_rules! impl_qvariant_value {
    ( $typeName:ty, $module:ident ) => {
        mod $module;

        impl QVariantValue for $typeName {
            fn can_convert(variant: &QVariant) -> bool {
                $module::can_convert(variant)
            }

            fn construct(value: &Self) -> QVariant {
                $module::construct(value)
            }

            fn value_or_default(variant: &QVariant) -> Self {
                $module::value_or_default(variant)
            }
        }
    };
}

impl_qvariant_value!(QUuid, qvariant_quuid);
