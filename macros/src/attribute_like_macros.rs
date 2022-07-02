use route_attribute_macro::route;
// Attribute-like macros

#[route(GET, "/")]
fn index() {
    // ...
}

pub fn run() {}
