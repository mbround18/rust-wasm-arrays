use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::Pet;


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Pets {
    list: Vec<Pet>,
}

#[wasm_bindgen]
impl Pets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Pets {
        Pets {
            list: Vec::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn list(&mut self) -> js_sys::Array {
        self.list
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect::<js_sys::Array>()
    }

    #[wasm_bindgen(method)]
    pub fn add(&mut self, pet: Pet) {
        self.list.push(pet);
    }

    #[wasm_bindgen(method)]
    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }   
}
