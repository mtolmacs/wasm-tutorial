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
    //alert(format!("Hello {} with id {}!", person.name, person.id).as_str());
    //panic!("Aaaaaaa");
    web_sys::console::log_1(&format!("Hello {}!", person.id).into());
}

#[wasm_bindgen]
pub fn owned_mutable_greet(mut p: String) {
    p.insert_str(0, "[LOG] ");

    // alert(p.as_str());
    web_sys::console::log_1(&p.as_str().into());
}

#[wasm_bindgen]
extern "C" {
    pub type TSDef;

    // Uses the JS "get()" method which is
    // provided by the "class" base prototype chain
    #[wasm_bindgen(method, getter)]
    fn id(this: &TSDef) -> String;

    // Uses the JS "set()" method which is
    // provided by the "class" base prototype chain
    // NOTE: "set_<property>" naming is important!
    #[wasm_bindgen(method, setter)]
    fn set_id(this: &TSDef, val: &str);

    #[wasm_bindgen(method)]
    fn run(this: &TSDef);
}

#[wasm_bindgen]
pub fn remote_instance_param(tsdef: &TSDef) {
    // Display the id of the instance
    //alert(tsdef.id().as_str());
    web_sys::console::log_1(&tsdef.id().as_str().into());

    // Modify the id on the JS instance
    tsdef.set_id("zyxw");

    // Call a method on the JS instance
    tsdef.run();

    alert("...");

    panic!("Ugh")
}
