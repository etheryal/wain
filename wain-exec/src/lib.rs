#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro)]
#![no_std]

extern crate no_std_compat as std;
extern crate wain_ast;

use std::prelude::v1::*;

pub mod trap;

mod cast;
mod globals;
mod import;
mod memory;
mod runtime;
mod stack;
mod table;
mod value;
mod yield_now;

#[cfg(feature = "std")]
pub use import::DefaultImporter;
pub use import::{check_func_signature, ImportInvalidError, ImportInvokeError, Importer};
pub use memory::Memory;
pub use runtime::Runtime;
pub use stack::Stack;
pub use value::Value;

use trap::Result;
use wain_ast::Module;

/// A convenient function to execute a WebAssembly module.
///
/// This function takes parsed and validated WebAssembly module and it invokes a start function if
/// presents. Otherwise it invokes a function exported as '_start' with no argument.
///
/// For standard I/O speed, this function locks io::Stdin and io::Stdout objects because currently
/// getchar() and putchar() don't buffer its input/output. This behavior may change in the future.
///
/// If the behavior is not acceptable, please make an abstract machine runtime with
/// Runtime::instantiate.
///
/// You will need importer for initializing Runtime struct. Please use DefaultImporter::with_stdio()
/// or make your own importer struct which implements Importer trait.
#[cfg(feature = "std")]
pub async fn execute(module: &Module<'_>) -> Result<()> {
    use std::io;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let importer = DefaultImporter::with_stdio(stdin.lock(), stdout.lock());
    let mut runtime = Runtime::instantiate(module, importer, core::u16::MAX)?;
    runtime.invoke_start().await?;
    Ok(())
}
