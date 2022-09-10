mod register;
pub use register::register;

mod start;
pub use start::start;

mod end;
pub use end::end;

pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;
