mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug , PartialEq, Eq)]
pub enum cell {
    Dead = 0,
    Alive = 1,
}

pub struct universe {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}
