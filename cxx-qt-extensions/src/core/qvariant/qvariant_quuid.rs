#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        include!("cxx-qt-extensions/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    unsafe extern "C++" {
        include!("cxx-qt-extensions/quuid.h");
        type QUuid = crate::QUuid;
    }

    #[namespace = "rust::cxxqtextensions1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QUuid"]
        fn qvariantCanConvertQUuid(variant: &QVariant) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "construct_QUuid"]
        fn qvariantConstruct(value: &QUuid) -> QVariant;
        #[rust_name = "value_or_default_QUuid"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QUuid;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QUuid(variant)
}

pub(crate) fn construct(value: &ffi::QUuid) -> ffi::QVariant {
    ffi::construct_QUuid(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QUuid {
    ffi::value_or_default_QUuid(variant)
}
