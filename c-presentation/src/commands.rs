mod register;
mod start;

pub use register::register;
pub use start::start;

pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;
