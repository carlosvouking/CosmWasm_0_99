pub mod contract;  // the logic behind the polling system seats here.
mod error;         // for my own customized errors (like 404 not found) on the web etc...
pub mod msg;       //  msgs endpoint are defined here...msgs actually make txns
pub mod state;     // underlying storage in the contract.

pub use crate::error::ContractError;
