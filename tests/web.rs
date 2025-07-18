
#![cfg(target_arch = "wasm32")]

extern crate gol;
use gol::Universe;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2),(3,3)]);
    universe
}

pub fn estimated_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,0), (2,1), (2,2), (3,4)]);
    universe 
}

[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    let estimated_spaceship = estimated_spaceship();

    input_universe.tick();
    assert_eq(&input_spaceship.get_cells(), &estimated_spaceship.get_cells());
}