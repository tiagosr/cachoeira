#[macro_use]
pub extern crate cgmath;
pub extern crate shred;
pub extern crate specs;

#[macro_use]
extern crate error_chain;
extern crate fnv;
extern crate hibitset;
#[macro_use]
extern crate log;
extern crate rayon;
#[macro_use]
extern crate serde;
extern crate specs_hierarchy;

#[macro_use]
#[cfg(feature = "profiler")]
extern crate thread_profiler;

pub use timing::*;

use std::sync::Arc;

pub mod timing;
pub mod console;
pub mod engine_system;

pub type ThreadPool = Arc<rayon::ThreadPool>;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/