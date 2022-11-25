

#[derive(Debug, Clone)]
pub struct Owners49 {
    table: Vec<i64>,
    total_step: i32,
    is_valid : bool
}

// I need only 7*7 bits for each step, then 64 bits is enogh


mod constructor;
mod methods;
mod indexing;
mod to_list;