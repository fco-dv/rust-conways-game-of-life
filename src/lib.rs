mod utils;
use std::fmt;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     let alert_message = format!("{name} Hello, wasm-game-of-life!");
//     alert(&alert_message);
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe{
        let width=64;
        let height=64;
         let cells: Vec<_> = (0..width*height)
         .map(|idx| {
            if idx%2 == 0 || idx%7 == 0 {
                Cell::Alive
            }else {
                Cell::Dead
            }
         } )
         .collect();

        Universe { width: (width), height: (height), cells: (cells) }

    }

    pub fn render(&self)->String {
        self.to_string()
    }

    fn get_index(&self, row:u32, col:u32) -> usize {
        //(row*self.width+col).try_into().unwrap()
        (row*self.width+col) as usize
    }

    fn live_neighbour_count(&self, row:u32, col:u32) -> u8 {
        let mut count = 0;
        //iterate on the 8  neighbourhood 
        for delta_row in [self.height -1, 0, 1].iter().cloned() {
            for delta_col in [ self.width -1 , 0, 1 ].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row)%self.height;
                let neighbour_col = (col + delta_col)%self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8; 
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_count = self.live_neighbour_count(row, col);
                
                let next_cell= match (cell, live_count) {
                    (Cell::Alive, x ) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                let symbol = if *cell ==  Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
      Ok(())  
    }
}