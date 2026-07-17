#![allow(clippy::field_reassign_with_default, clippy::let_underscore_future)]
extern crate self as sydar_cli;

mod cli;
pub mod error;
pub mod extensions;
mod helpers;
mod imports;
mod matchers;
pub mod modules;
mod notifier;
pub mod result;
pub mod utils;
mod wizards;

pub use cli::{Options, sydarCli, TerminalOptions, TerminalTarget, sydar_cli};
pub use workflow_terminal::Terminal;
