use std::path::Path;

pub trait FileLoadable {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized;
}
