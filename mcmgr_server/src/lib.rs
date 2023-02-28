pub mod prelude {
    pub use anyhow::{Context, Result};
    pub use std::{
        ffi::{OsStr, OsString},
        io::{BufRead, BufReader, BufWriter},
        path::{Path, PathBuf},
        process::{Child, Command, Stdio},
        vec,
    };
}

mod instance;
mod event;
pub use instance::Instance;