pub mod entrypoint;
pub mod instruction;
pub mod error;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
use include_idl::include_idl;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
include_idl!();

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "McBurn",
    project_url: "https://https://github.com/honeygrahams2/mcburn-rs",
    contacts: "email:nathan@airadlabs.com",
    policy: "https://github.com/honeygrahams2/mcburn-rs/SECURITY.md",

    // Optional Fields
    source_code: "https://https://github.com/honeygrahams2/mcburn-rs"
}
