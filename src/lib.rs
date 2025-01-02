mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Person {
    pub id: u32,
    name: String, // String is not Copy, so we cannot make it public!
}

// Implement a constructor and a getter/setter for the String field
#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, name: String) -> Person {
        Person { id, name }
    }

    // Getter which automatically gets called on the JS side
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    // Setter which automatically gets called on the JS side
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[wasm_bindgen]
pub fn greet(person: &Person) {
    alert(format!("Hello {} with id {}!", person.name, person.id).as_str());
}
