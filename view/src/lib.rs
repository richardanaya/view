#![no_std]
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(fake_call_site)]
pub use view_macro::view;