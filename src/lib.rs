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

#[wasm_bindgen]
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
            let idx = self.get_index(row, col);
            let cell = self.cells[idx];
            let live_neighbours = self.live_neighbours_count(row, col);

            let next cell = match (cell , live_neighbours) {
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                (Cell::Dead, 3) => Cell::Alive,
                (otherwise, _) => otherwise,
            };

            next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}