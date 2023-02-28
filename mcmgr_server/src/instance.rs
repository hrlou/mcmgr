use std::io::Write;

use crate::prelude::*;
pub use crate::*;

pub struct Instance {
    path: PathBuf,
    java: OsString,
    args: Vec<OsString>,
    pub process: Option<Box<Child>>,
}

macro_rules! unwrap_log {
    ($kind:expr, $e:expr) => {
        match $kind {
            "ERROR" => log::error!($e),
            "WARN" => log::warn!($e),
            "DEBUG" => log::debug!($e),
            _ => log::info!($e),
        }
    };
}

impl Instance {
    /// Creates a new minecraft server instance
    /// 
    /// ```rust
    /// let server = Instance::new("./server", "/usr/bin/java", "server.jar", vec!["nogui"]);
    /// ```
    /// 
    pub fn new<P, S, V>(path: P, java: S, jar: S, jvm: V) -> Result<Instance>
    where
        P: Into<PathBuf>,
        S: Into<OsString>,
        V: Into<Vec<S>>,
    {
        // Convert into
        let (path, java, jar, jvm) = (path.into(), java.into(), jar.into(), jvm.into());
        // Add path check~
        // Create arguments
        let mut args: Vec<OsString> = vec!["-jar".into(), jar];
        // There's probably a better way to do this
        for arg in jvm {
            let arg = arg.into();
            args.push(arg)
        }
        Ok(Instance {
            path,
            java,
            args,
            process: None,
        })
    }

    pub fn spawn(&mut self) -> Result<()> {
        self.process = Some(Box::new(
            Command::new(&self.java)
                .current_dir(&self.path)
                .args(&self.args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?,
        ));
        let process = self.process.as_mut().context("process isn't running")?;
        let stdout = process.stdout.take().context("cannot access stdout")?;
        std::thread::spawn(move || -> Result<()> {
            let buf_reader = BufReader::new(stdout);
            for l in buf_reader.lines() {
                let l = l.context("line cannot be read")?;
                Self::process_line(l);
            }
            Ok(())
        });
        Ok(())
    }

    /// Process a line from the minecraft server
    fn process_line(l: String) {
        let re = regex::Regex::new(r"(\[(?P<time>[0-9:]+)?\])? ?(\[(?P<from>.*)/(?P<kind>[A-Z]+)?\]: )? ?(?P<msg>.*)").unwrap();
        let l = re.captures(&l).unwrap();
        let target = match l.name("from") {
            Some(x) => x.as_str(),
            _ => "server",
        };
        let kind = match l.name("kind") {
            Some(x) => x.as_str(),
            _ => "INFO",
        };
        match kind {
            "ERROR" => log::error!(target: target, "{}", &l["msg"]),
            "WARN" => log::warn!(target: target, "{}", &l["msg"]),
            "DEBUG" => log::debug!(target: target, "{}", &l["msg"]),
            _ => log::info!(target: target, "{}", &l["msg"]),
        }
    }

    /// Execute a command on the server instance
    pub fn execute<S: Into<Vec<u8>>>(&mut self, s: S) -> Result<()> {
        let s = s.into();
        let process = self.process.as_mut().context("process isn't running")?;
        let mut stdin = process.stdin.take().context("cannot access stdin")?;
        log::debug!("Writing to process: {:?}", s);
        stdin.write(&s)?;
        Ok(())
    }
}