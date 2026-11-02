//! One module per check. Each check produces a `Report` and owns no UI.
pub mod canary;
pub mod changes;
pub mod denials;
pub mod doctor;
pub mod inspect;
pub mod integrations;
pub mod network;
pub mod packages;
pub mod pdf;
pub mod receipt;
pub mod route;
pub mod scan;
pub mod setup;
pub mod shell;
