use crate::QUuid;
use cxx::type_id;
use cxx_qt_lib::{QVector, QVectorElement};

macro_rules! impl_qvector_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        mod $module;

        impl QVectorElement for $typeName {
            type TypeId = type_id!($typeId);

            fn append(vector: &mut QVector<Self>, value: Self) {
                $module::append(vector, &value);
            }

            fn append_clone(vector: &mut QVector<Self>, value: &Self) {
                $module::append(vector, value);
            }

            fn clear(vector: &mut QVector<Self>) {
                $module::ffi::cxx_clear(vector)
            }

            fn clone(vector: &QVector<Self>) -> QVector<Self> {
                $module::clone(vector)
            }

            fn contains(vector: &QVector<Self>, value: &Self) -> bool {
                $module::ffi::cxx_contains(vector, value)
            }

            fn default() -> QVector<Self> {
                $module::default()
            }

            fn drop(vector: &mut QVector<Self>) {
                $module::drop(vector);
            }

            unsafe fn get_unchecked(vector: &QVector<Self>, pos: isize) -> &Self {
                $module::get_unchecked(vector, pos)
            }

            fn index_of(vector: &QVector<Self>, value: &Self) -> isize {
                $module::index_of(vector, value)
            }

            fn insert(vector: &mut QVector<Self>, pos: isize, value: Self) {
                $module::insert(vector, pos, &value);
            }

            fn insert_clone(vector: &mut QVector<Self>, pos: isize, value: &Self) {
                $module::insert(vector, pos, value);
            }

            fn len(vector: &QVector<Self>) -> isize {
                $module::len(vector)
            }

            fn remove(vector: &mut QVector<Self>, pos: isize) {
                $module::remove(vector, pos);
            }

            fn reserve(vector: &mut QVector<Self>, size: isize) {
                $module::reserve(vector, size);
            }
        }
    };
}

impl_qvector_element!(QUuid, qvector_quuid, "QVector_QUuid");
