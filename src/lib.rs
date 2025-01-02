mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Person {
    pub id: u32,
}

// Implement a constructor for the Person struct
#[wasm_bindgen]
impl Person {
    // Need to explicitly flag a method as the JS constructor
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32) -> Person {
        Person { id }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str, person: &Person) {
    alert(format!("Hello {} with id {}!", name, person.id).as_str());
}
