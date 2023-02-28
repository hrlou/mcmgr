use mcmgr_server::{Instance};
use anyhow::Result;
use std::io::{self, prelude::*};

fn main() -> Result<()> {
    // use std::io::BufRead;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let mut server = Instance::new("./server", "java", "server.jar", vec!["nogui"])?;
    server.spawn()?;
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        let mut handle = stdin.lock();
        io::stdout().flush()?;
        handle.read_line(&mut input)?;
        if input == "stop" {
            break;
        }
        server.execute(input)?;
    }
    server.process.unwrap().wait()?;
    Ok(())
}
