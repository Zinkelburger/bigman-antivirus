//! Bigman's checks, report model and private store. No binaries and no UI live here:
//! the CLI, daemon and native host are thin clients of this library.
pub mod actions;
pub mod checks;
pub mod ipc;
pub mod model;
pub mod rules;
pub mod runner;
pub mod store;
