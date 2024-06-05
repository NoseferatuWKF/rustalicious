// mod blanket_implementation;
mod zero_cost_abstraction;
mod interior_mutability;
use crate::interior_mutability::example::refcell_example;
use crate::zero_cost_abstraction::example::polymorphism_example;

fn main() {
    refcell_example();
    polymorphism_example();
}
