mod zero_cost_abstraction;
mod interior_mutability;
use interior_mutability::example::refcell_example;
use zero_cost_abstraction::example::polymorphism_example;

fn main() {
    refcell_example();
    polymorphism_example();
}
