use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Binding makes the Rust code available to JavaScript.
#[wasm_bindgen]
// derive the Serialize and Deserialize traits will make it possible to serialize and deserialize Rust structs to and from JSON.
#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    name: String,
    age: u8,
}

#[wasm_bindgen]
impl Pet {
    // The `new` function is exported to JavaScript as a constructor.
    #[wasm_bindgen(constructor)]
    pub fn new(data: JsValue) -> Pet {
        data.into_serde().unwrap()
    }

    // The `name` getter is exported to JavaScript.
    #[wasm_bindgen(getter)]
    pub fn get_name(self) -> String {
        self.name
    }

    // The `set_name` function is exported to JavaScript.
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

        // The `name` getter is exported to JavaScript.
    #[wasm_bindgen(getter)]
    pub fn get_age(self) -> u8 {
        self.age
    }

    // The `set_name` function is exported to JavaScript.
    #[wasm_bindgen(setter)]
    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}
