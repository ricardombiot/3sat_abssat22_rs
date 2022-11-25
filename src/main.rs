pub mod abssat {
    pub mod gpath;
    pub mod utils;
    pub mod sat_machine;
    pub mod reader;
    pub mod reader_exp;
}

pub mod program;

#[cfg(test)]
mod test;

use crate::program::execute;
use std::io;
fn main() -> io::Result<()> {
    execute()
}

