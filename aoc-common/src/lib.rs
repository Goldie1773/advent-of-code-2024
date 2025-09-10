use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).expect("failed to read file")
}

#[macro_export]
macro_rules! read_file_manifest {
    ($rel_path:expr) => {
        $crate::read_file(concat!(env!("CARGO_MANIFEST_DIR"), "/", $rel_path))
    };
}