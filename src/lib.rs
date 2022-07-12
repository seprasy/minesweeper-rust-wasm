extern "C" {
    pub fn fillRect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32, a: i32);
    pub fn fillNum(x: i32, y: i32, num: i32);
    pub fn rand(max: usize) -> usize;
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    unsafe {
        fillRect(0, 0, 30, 30, 50, 50, 50, 50);
        fillNum(10, 20, 1);
    }
    x + 10
}

#[derive(Clone, Debug)]
pub struct Cell {
    cell_type: CellType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Bomb,
    None,
}

#[derive(Clone, Debug)]
pub struct Minesweeper {
    name: String,
    screen_width: i32,
    screen_height: i32,
    cell_size: i32,
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>,
    cell_border: i32,
    bomb_percent: usize,
}

impl Minesweeper {
    fn get_empty_cell(&mut self) -> (usize, usize) {
        let total_cells = self.rows * self.cols;
        let num = unsafe { rand(total_cells-1) };
        let row = num / self.cols;
        let col = num % self.cols;

        if self.cells[row][col].cell_type == CellType::None {
            return (row, col);
        }

        return self.get_empty_cell();
    }

    fn generate_bombs(&mut self) {
        let total_cells = self.rows * self.cols;
        let mut no_of_bombs = total_cells * self.bomb_percent / 100;

        while no_of_bombs > 0 {
            let (row, col) = self.get_empty_cell();
            self.cells[row][col].cell_type = CellType::Bomb;
            no_of_bombs -= 1;
        }
    }

    fn neighbours_with_bombs(&self, row: usize, col: usize) -> i32 {
        let mut bombs = 0;
        for row_offset in -1..2 {
            for col_offset in -1..2 {
                if !(col_offset == 0 && row_offset == 0) {
                    if !(row as i32 + row_offset >= self.rows as i32
                        || col as i32 + col_offset >= self.cols as i32
                        || row as i32 + row_offset < 0
                        || col as i32 + col_offset < 0)
                    {
                        if self.cells[(row as i32+ row_offset) as usize][(col as i32 + col_offset) as usize]
                            .cell_type
                            == CellType::Bomb
                        {
                            bombs += 1;
                        }
                    }
                }
            }
        }
        return bombs;
    }

    fn render(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.cells[row][col].cell_type {
                    CellType::Bomb => unsafe {
                        fillRect(
                            col as i32 * self.cell_size + self.cell_border,
                            row as i32 * self.cell_size + self.cell_border,
                            self.cell_size - self.cell_border,
                            self.cell_size - self.cell_border,
                            200,
                            50,
                            50,
                            255,
                        );
                    },
                    CellType::None => unsafe {
                           fillRect(
                            col as i32 * self.cell_size + self.cell_border,
                            row as i32 * self.cell_size + self.cell_border,
                            self.cell_size - self.cell_border,
                            self.cell_size - self.cell_border,
                            200,
                            200,
                            200,
                            255,
                        );
                        let neighbouring_bombs = self.neighbours_with_bombs(row, col);
                        if neighbouring_bombs > 0 {
                            fillNum(
                                (col as i32 * self.cell_size )+ ( self.cell_size / 4) ,
                                (row as i32 * self.cell_size ) + (self.cell_size *3 / 4),
                                neighbouring_bombs,
                            );
                        } 
                    },
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn init(screen_height: i32, screen_width: i32, cell_size: i32) -> *mut Minesweeper {
    let rows = (screen_height / cell_size) as usize;
    let cols = (screen_width / cell_size) as usize;
    let cells = vec![
        vec![
            Cell {
                cell_type: CellType::None,
            };
            cols
        ];
        rows
    ];
    let mut game = Minesweeper {
        name: "Game on !".to_string(),
        screen_height: screen_height,
        screen_width: screen_width,
        cell_size: cell_size,
        rows: rows,
        cols: cols,
        cells: cells,
        cell_border: 2,
        bomb_percent: 15,
    };

    game.generate_bombs();
    return Box::into_raw(Box::new(game));
}

#[no_mangle]
pub extern "C" fn render(game: *mut Minesweeper) {
    let game = unsafe { &mut *game };

    unsafe {
        fillRect(0, 0, game.screen_width, game.screen_height, 20, 20, 20, 255);
    }

    game.render();
}
