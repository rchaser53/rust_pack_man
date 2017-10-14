use std::fs::File;
use std::io::prelude::*;

use sdl2::{render, video};

use constants::Direction::{East, West, South, North};
use error_handling::{Result as CustomResult, GameOverError};
use collision_handler::{CollisionFrame};
use game_status::{GameStatus};
use circle::{Circle};
use game_field::field_row::FieldRow;
use game_field::field_cell::CellType;

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
pub const CELL_WIDTH: i16 = 30;
pub const CELL_HEIGHT: i16 = 30;
pub const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
pub const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

const SQUARE_MAP_PATH: &'static str = "assets/maps/sample_map1.txt";

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

pub struct Field {
    pub field_rows: Vec<FieldRow>,
    pub circle: Circle,
    pub game_status: GameStatus
}

impl Field  {
    pub fn new() -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();

        let map_config = read_file(SQUARE_MAP_PATH);
        let row_defs: Vec<&str> = map_config.split("\n").collect();
        let row_defs_length = row_defs.len();
        for row_index in 0..row_defs_length {
            rows.push(FieldRow::new(&row_defs[row_index], row_index));
        }

        return Field {
            field_rows: rows,
            circle: Circle::new(),
            game_status: GameStatus::new()
        };
    }

    pub fn renew(&mut self, renderer: &mut render::Canvas<video::Window>) -> CustomResult<()> {
        if self.game_status.is_pause { return Ok(()); }
        self.draw_row(renderer);
        
        let current_cell_type;
        {
            current_cell_type = self.get_current_cell_type()?;
        }
        {
            self.take_action_from_cell_type(current_cell_type)?;
        }

        return Ok(self.circle.renew(renderer));
    }

    pub fn draw_row(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        let rows = self.field_rows.iter();

        for row in rows {
            let cells = row.field_cells.iter();
            for cell in cells {
                cell.draw(renderer);
            }
        }
    }

    pub fn get_next_cell_index(&self) -> (i16, i16) {
        let column: f32 = (self.circle.x * COLUMUNS_NUMBER) as f32 / SCREEN_WIDTH as f32;
        let row: f32 = (self.circle.y * ROWS_NUMBER) as f32 / SCREEN_HEIGHT as f32;

        return match self.circle.direction {
            num if num == East.value() => (row as i16, column.round() as i16),
            num if num == South.value() => (row.round() as i16, column as i16),
            num if num == West.value() => (row as i16, column.round() as i16 - 1),
            num if num == North.value() => (row.round() as i16 - 1, column as i16),
            _ => (column as i16, row as i16)
        };
    } 

    pub fn get_current_cell_type(&self) -> Result<CellType, GameOverError> {
        let (row, column) = self.get_next_cell_index();

        if self.is_outof_frame(row, column) {
            return Err(GameOverError::OtherError("out of the frame"));
        }

        return Ok(self.field_rows[row as usize].field_cells[column as usize].status.cell_type);
    }

    pub fn is_outof_frame(&self, row: i16, column: i16) -> bool {
        return row < 0 || (self.field_rows.len() as i16 - 1) < row
                || column < 0 || (self.field_rows[0].field_cells.len() as i16 - 1) < column;
    }

    pub fn take_action_from_cell_type(&mut self, current_cell_type: CellType) -> Result<(), GameOverError> {
        match current_cell_type {
            CellType::Damage => Err(CollisionFrame::hit_enemy()),
            CellType::Wall => {
                self.circle.is_stoped = true;
                return Ok(());
            },
            _ => {
                self.circle.is_stoped = false;
                return Ok(());
            }
        }
    }
}