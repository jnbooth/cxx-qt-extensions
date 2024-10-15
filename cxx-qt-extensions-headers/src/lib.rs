use std::{fs::File, io::Write, path::Path};

pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory)
        .expect("Could not create cxx-qt-extensions header directory");
    for (file_contents, file_name) in [
        (include_str!("../include/core/qlist.h"), "qlist.h"),
        (include_str!("../include/core/qset.h"), "qset.h"),
        (include_str!("../include/core/quuid.h"), "quuid.h"),
        (include_str!("../include/core/qvariant.h"), "qvariant.h"),
        (include_str!("../include/core/qvector.h"), "qvector.h"),
    ] {
        let h_path = format!("{}/{file_name}", directory.display());
        let mut header = File::create(h_path).expect("Could not create cxx-qt-extensions header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt-extensions header");
    }
}
