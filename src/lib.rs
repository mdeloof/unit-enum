//! From an existing enum, derive a unit `enum` where none of the
//! variants have any fields.
//!
//! ```
//! use unit_enum::UnitEnum;
//!
//! #[derive(UnitEnum)]
//! #[unit_enum(name = "AllType", derive(Clone, Copy))]
//! enum All {
//!     Unit,
//!     Tuple(bool, i32),
//!     Struct { x: i32, y: i32 },
//! }
//! ```
//!
//! Will generate:
//!
//! ```
//! #[derive(Clone, Copy)]
//! enum AllType {
//!     Unit,
//!     Tuple,
//!     Struct,
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod analyze;
mod codegen;
mod lower;
mod parse;

use analyze::analyze;
use codegen::codegen;
use lower::lower;
use parse::parse;

#[proc_macro_derive(UnitEnum, attributes(unit_enum))]
#[proc_macro_error]
pub fn unit_enum(input: TokenStream) -> TokenStream {
    let item_enum = parse(input.into());
    let model = analyze(item_enum);
    let ir = lower(model);
    let rust = codegen(ir);
    rust.into()
}
