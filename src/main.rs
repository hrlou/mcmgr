use std::vec;
#[allow(unused_imports)]
use std::{
    env,
    error::Error,
    ffi::{OsStr, OsString},
    io::{self, BufRead, BufReader},
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

pub struct ServerInstance {
    path: PathBuf,
    args: Vec<OsString>,
    process: Option<Box<Child>>,
}

impl ServerInstance {
    pub fn init<P, S, V>(path: P, jar: S, jvm: V) -> Result<ServerInstance, Box<dyn Error>>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
        V: AsRef<Vec<S>>,
    {
        // Convert reference to PathBuf
        let path = path.as_ref().to_path_buf();
        // Create arguments
        let mut args: Vec<OsString> = vec!["-jar".into(), jar.as_ref().into()];
        // There's probably a better way to do this
        for arg in jvm.as_ref() {
            let arg = arg.as_ref().to_os_string();
            args.push(arg)
        }
        Ok(ServerInstance {
            path,
            args,
            process: None,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.process = Some(Box::new(
            Command::new("java")
                .current_dir(&self.path)
                .args(&self.args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?,
        ));
        Ok(())
    }

    pub fn stdout(&mut self) -> Result<i32, Box<dyn Error>> {
        let process = self.process.as_mut().unwrap();
        match process.stdout.as_mut() {
            Some(out) => {
                let buf_reader = BufReader::new(out);
                for line in buf_reader.lines() {
                    match line {
                        Ok(l) => return Ok(0),
                        Err(_) => return Ok(1),
                    };
                };
                Ok(0)
            }
            None => return Ok(1),
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let mut server = ServerInstance::init("./server", "server.jar", vec!["nogui"])?;
    server.start()?;
    server.stdout()?;
    Ok(())
}
