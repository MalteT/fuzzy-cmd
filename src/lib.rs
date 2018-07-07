//! # Example
//! ```
//! use fuzzy_cmd::FuzzyCmd;
//! use std::env::args;
//! use std::iter::FromIterator;
//!
//! fn main() {
//!     let mut fuzz = FuzzyCmd::new().enable_fuzzy();
//!
//!     {
//!         let mut help = fuzz.add("help");
//!         help.add("all")
//!             .call(|| println!("Auto help generator will be done soon.."));
//!     }
//!
//!     {
//!         let mut n_bake = fuzz.add("bake");
//!         n_bake.add("cake").call(|| bake("cake"));
//!         n_bake.add("oven").call(|| bake("oven"));
//!         n_bake.add("pizza").call(|| bake("pizza"));
//!     }
//!     {
//!         let build = fuzz.add("build");
//!     }
//!     let cmd = String::from_iter(args());
//!     fuzz.exec(&cmd);
//! }
//!
//! fn bake(recipe: &str) {
//!     println!(
//!         "Baking {}",
//!         match recipe {
//!             "cake" => "a cake. But finish your tests first!",
//!             "oven" => "myself, obviously... who whould do that to an oven?",
//!             "pizza" => "pizza..",
//!             _ => panic!("Bug"),
//!         }
//!     );
//! }
//! ```

mod fuzzy_cmd;

pub use fuzzy_cmd::FuzzyCmd;
pub use fuzzy_cmd::Node;
