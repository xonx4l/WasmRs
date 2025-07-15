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

impl universe {
    fn get_index(&self , width: u32,height: u32) -> usize {
      (row * self.width + column) as usize 
    }

    fn live_neighbour_count(&self , row: u32 , column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1,0,1].iter().cloned() {
            for delta_col in [self.width - 1,0,1].iter.cloned(){
                if delta_row == 0 && delta_col ==0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_column = (column + delta_col) % self.width;
                let idx = self.get.index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}