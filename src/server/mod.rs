pub mod prelude {
    pub use anyhow::{Context, Result};
    pub use std::{
        ffi::{OsStr, OsString},
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
        process::{Child, Command, Stdio},
        vec,
    };
}

pub mod instance;
pub use instance::Instance;