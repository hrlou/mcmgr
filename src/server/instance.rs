use super::prelude::*;

pub struct Instance {
    path: PathBuf,
    java: OsString,
    args: Vec<OsString>,
    pub process: Option<Box<Child>>,
}

impl Instance {
    pub fn init<P, S, V>(path: P, java: S, jar: S, jvm: V) -> Result<Instance>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
        V: AsRef<Vec<S>>,
    {
        // Convert reference to PathBuf
        let path = path.as_ref().to_path_buf();
        let java = java.as_ref().to_os_string();
        // Create arguments
        let mut args: Vec<OsString> = vec!["-jar".into(), jar.as_ref().into()];
        // There's probably a better way to do this
        for arg in jvm.as_ref() {
            let arg = arg.as_ref().to_os_string();
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
        self.stdout()?;
        Ok(())
    }

    /// Process a line from the minecraft server
    fn process_line(l: String) {
        log::info!("{}", l);
    }


    /// Reads the stdout of the minecraft server
    fn stdout(&mut self) -> Result<()> {
        let process = self.process.as_mut().context("process isn't running")?;
        let stdout = process.stdout.as_mut().context("cannot read stdout")?;
        let buf_reader = BufReader::new(stdout);
        for l in buf_reader.lines() {
            let l = l.context("line cannot be read")?;
            Self::process_line(l);
        }
        Ok(())
    }

    pub fn execute<S: AsRef<OsStr>>(&mut self, s: S) -> Result<()> {
        // let process = self.process.as_mut().context("process isn't running")?;
        // let stdin = process.stdin.as_mut().context("cannot read stdout")?;
        Ok(())
    }
}