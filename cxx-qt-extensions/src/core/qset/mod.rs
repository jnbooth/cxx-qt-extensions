use crate::QUuid;
use cxx::type_id;
use cxx_qt_lib::{QSet, QSetElement};

macro_rules! impl_qset_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        mod $module;

        impl QSetElement for $typeName {
            type TypeId = type_id!($typeId);

            fn clear(set: &mut QSet<Self>) {
                $module::ffi::cxx_qset_clear(set)
            }

            fn clone(set: &QSet<Self>) -> QSet<Self> {
                $module::clone(set)
            }

            fn contains(set: &QSet<Self>, value: &Self) -> bool {
                $module::ffi::cxx_qset_contains(set, value)
            }

            fn default() -> QSet<Self> {
                $module::default()
            }

            fn drop(set: &mut QSet<Self>) {
                $module::drop(set);
            }

            unsafe fn get_unchecked(set: &QSet<Self>, pos: isize) -> &Self {
                $module::get_unchecked(set, pos)
            }

            fn insert(set: &mut QSet<Self>, value: Self) {
                $module::insert(set, &value);
            }

            fn insert_clone(set: &mut QSet<Self>, value: &Self) {
                $module::insert(set, value);
            }

            fn len(set: &QSet<Self>) -> isize {
                $module::len(set)
            }

            fn remove(set: &mut QSet<Self>, value: &Self) -> bool {
                $module::ffi::cxx_qset_remove(set, value)
            }
        }
    };
}

impl_qset_element!(QUuid, qset_quuid, "QSet_QUuid");
