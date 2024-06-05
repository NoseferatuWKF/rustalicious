// let's say we have a shared trait/interface
trait Transport {
    fn travels_on(&self) -> String;
}

// and then we do the concrete implementation on a struct
struct Car;
impl Transport for Car {
    fn travels_on(&self) -> String {
        String::from("road")
    }
}

// and on another struct
struct Train;
impl Transport for Train {
    fn travels_on(&self) -> String {
        String::from("rails")
    }
}

pub fn polymorphism_example() {
    let car = Car;
    let train = Train;
    println!("{}", dynamic_dispatch(&car));
    println!("{}", static_dispatch(&train));
}

// we need to be explicit on whether to do dynamic dispatch...
fn dynamic_dispatch(transport: &dyn Transport) -> String {
    transport.travels_on()
}

// ... or static dispatch
fn static_dispatch(transport: &impl Transport) -> String {
    transport.travels_on()
}

