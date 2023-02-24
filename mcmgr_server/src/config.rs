use crate::prelude::*;

pub struct Config {
    pub path: PathBuf,
    pub java: OsString,
    pub jvm_args: Vec<OsString>,
}