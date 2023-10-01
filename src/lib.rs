#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod project;
mod scripting;
mod tabs;
mod theme;
pub use app::AnimaApp;

pub type Result = anyhow::Result<(), String>;
