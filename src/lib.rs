use std::path::{Path, PathBuf};

pub fn get_coredump_path<P: AsRef<Path>>(name: P) -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(name.as_ref())
        .with_extension("core");
    path.to_owned()
}

