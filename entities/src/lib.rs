#![feature(fmt_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(print_internals)]
#![feature(core_panic)]
#![feature(structural_match)]
//! ## Check Script
//! ```rs
//! cargo expand --lib user > re_user.rs
//! cargo expand --lib account::user::user > src/re_user.rs
//! cargo expand --lib t > src/qr.rs
//! ```
//! > no need path prefix due to `pub use` in lib.rs
//!
//! ## Check Fix with Reg
//! ```
//! #\[.*
//! ::core:: -- core::
//! ::std -- std
//! <:: -- <
//!  ::alloc -- alloc
//! ```
//! * add
//! ```
//! #![feature(fmt_internals)]
//! #![feature(fmt_helpers_for_derive)]
//! #![feature(print_internals)]
//! #![feature(core_panic)]
//! #![feature(structural_match)]
//! ```
//! ## It is need a OrmDataloader due to ths following
//! ```
//! cannot find type `OrmDataloader` in the crate root
//! not found in the crate root
//! ```
//!
//! # How to re-export
//! 1. `pub mod $schema` in mod.rs
//! 2. `mod module` and `pub use module::*`
pub use common::*;

mod account_model;
mod address_model;
pub use account_model::*;
pub use address_model::*;
pub mod user_ex;
// mod address;
// pub use address::*;
// pub mod ownership;

// pub mod inner_common;
