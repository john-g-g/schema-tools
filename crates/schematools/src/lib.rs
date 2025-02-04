#[macro_use]
extern crate lazy_static;

#[cfg(feature = "codegen")]
pub mod codegen;
pub mod discovery;
pub mod error;
pub mod hash;
pub mod process;
pub mod resolver;
pub mod schema;
pub mod scope;
pub mod storage;
pub mod tools;
pub mod validate;

pub const VERSION: &str = "0.13.1";
