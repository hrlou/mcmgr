pub mod server;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let mut server = server::Instance::init("java", "./server", "server.jar", vec!["nogui"])?;
    log::info!("Staring Minecraft Server");
    server.spawn()?;
    server.process.unwrap().wait()?;
    Ok(())
}
