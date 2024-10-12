use cxx::{type_id, ExternType};
use cxx_qt_lib::QString;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum UuidVariant {
        VarUnknown = -1,
        NCS = 0,
        DCE = 2,
        Microsoft = 6,
        Reserved = 7,
    }

    #[repr(i32)]
    enum UuidVersion {
        VerUnknown = -1,
        Time = 1,
        EmbeddedPOSIX = 2,
        Md5 = 3,
        Random = 4,
        Sha1 = 5,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-extensions/quuid.h");
        type QUuid = super::QUuid;
        type UuidVariant;
        type UuidVersion;

        #[rust_name = "is_null"]
        fn isNull(self: &QUuid) -> bool;

        fn variant(self: &QUuid) -> UuidVariant;

        fn version(self: &QUuid) -> UuidVersion;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "quuid_init_default"]
        fn construct() -> QUuid;

        // QUuid::QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8)
        #[doc(hidden)]
        #[rust_name = "quuid_init_fields"]
        fn construct(
            l: u32,
            w1: u16,
            w2: u16,
            b1: u8,
            b2: u8,
            b3: u8,
            b4: u8,
            b5: u8,
            b6: u8,
            b7: u8,
            b8: u8,
        ) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "qdate_to_qstring"]
        fn toQString(value: &QUuid) -> QString;
    }
    #[namespace = "rust::cxxqtextensions1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "quuid_new_v3"]
        fn quuidNewV3(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_new_v4"]
        fn quuidNewV4() -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_new_v5"]
        fn quuidNewV5(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_from_string"]
        fn quuidFromString(string: &QString) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_from_str"]
        fn quuidFromStr(string: &str) -> QUuid;
    }
}

pub use ffi::{UuidVariant, UuidVersion};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct QUuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Default for QUuid {
    fn default() -> Self {
        ffi::quuid_init_default()
    }
}

impl fmt::Display for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qdate_to_qstring(self))
    }
}

impl fmt::Debug for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl QUuid {
    pub fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        ffi::quuid_init_fields(
            d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7],
        )
    }

    pub fn as_fields(&self) -> (u32, u16, u16, &[u8; 8]) {
        (self.data1, self.data2, self.data3, &self.data4)
    }

    pub fn new_v3(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_new_v3(namespace, data)
    }

    pub fn new_v4() -> Self {
        ffi::quuid_new_v4()
    }

    pub fn new_v5(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_new_v5(namespace, data)
    }

    pub fn from_string(uuid: &QString) -> Option<Self> {
        let id = ffi::quuid_from_string(uuid);
        if !id.is_null() || is_null_uuid(&String::from(uuid)) {
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QUuidParseError;

impl Display for QUuidParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid UUID")
    }
}

impl Error for QUuidParseError {}

impl FromStr for QUuid {
    type Err = QUuidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = ffi::quuid_from_str(s);
        if !id.is_null() || is_null_uuid(s) {
            Ok(id)
        } else {
            Err(QUuidParseError)
        }
    }
}

unsafe impl ExternType for QUuid {
    type Id = type_id!("QUuid");
    type Kind = cxx::kind::Trivial;
}

fn is_null_uuid(uuid: &str) -> bool {
    uuid == "00000000-0000-0000-0000-000000000000"
        || uuid == "{00000000-0000-0000-0000-000000000000}"
}

#[cfg(feature = "uuid")]
impl From<Uuid> for QUuid {
    fn from(value: Uuid) -> Self {
        let (data1, data2, data3, data4) = value.as_fields();
        Self::from_fields(data1, data2, data3, data4)
    }
}

#[cfg(feature = "uuid")]
impl From<QUuid> for Uuid {
    fn from(value: QUuid) -> Self {
        let (data1, data2, data3, data4) = value.as_fields();
        Self::from_fields(data1, data2, data3, data4)
    }
}
