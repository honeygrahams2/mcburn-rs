pub mod entrypoint;
pub mod instruction;
pub mod error;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
use include_idl::include_idl;

#[cfg(not(feature = "no-entrypoint"))]
include_idl!();