mod archive;
pub(crate) mod common;
mod entrypoint;
mod grep;
mod init;
mod instructions;
mod list;
mod manifesto_instructions;
mod memory_instructions;
mod run;
mod show;
mod status;
pub(crate) mod trace;
mod update;
mod validate;
mod validate_repo;
mod worktree_wizard;

pub(crate) use entrypoint::main;
