extern "C" {
    pub fn fillRect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32, a: i32);
    pub fn fillNum(x: i32, y: i32, num: i32);
    pub fn rand(max: usize) -> usize;
    pub fn print(text: &str);
}

fn fill_rect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32, a: i32) {
    unsafe {
        fillRect(x, y, w, h, r, g, b, a);
    }
}

fn fill_num(x: i32, y: i32, num: i32) {
    unsafe {
        fillNum(x, y, num);
    }
}

fn info(text: &str) {
    unsafe {
        print(text);
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    cell_type: CellType,
    state: CellState,
    bomb_neighbours: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Open,
    Close,
    Marked,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Bomb,
    None,
}

#[derive(Clone, Debug)]
pub struct Minesweeper {
    screen_width: i32,
    screen_height: i32,
    cell_size: i32,
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>,
    cell_border: i32,
    bomb_percent: usize,
    generated: bool,
}

fn abs(num: i32) -> i32 {
    if num < 0 {
        num * -1
    } else {
        num
    }
}
impl Minesweeper {
    fn get_empty_cell(&mut self) -> (usize, usize) {
        let total_cells = self.rows * self.cols;
        let num = unsafe { rand(total_cells - 1) };
        let row = num / self.cols;
        let col = num % self.cols;

        if self.cells[row][col].cell_type == CellType::None {
            return (row, col);
        }

        return self.get_empty_cell();
    }
    fn generate_bombs(&mut self, c_row: usize, c_col: usize) {
        let total_cells = self.rows * self.cols;
        let mut no_of_bombs = total_cells * self.bomb_percent / 100;

        while no_of_bombs > 0 {
            let (row, col) = self.get_empty_cell();

            match (row, col) {
                i if abs(i.0 as i32 - c_row as i32) < 2 && abs(i.1 as i32  - c_col as i32) < 2 => continue,
                _ => {
                    self.cells[row][col].cell_type = CellType::Bomb;
                    no_of_bombs -= 1;
                }
            }
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                self.cells[row][col].bomb_neighbours = self.neighbours_with_bombs(row, col);
            }
        }
        self.generated = true;
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
                        if self.cells[(row as i32 + row_offset) as usize]
                            [(col as i32 + col_offset) as usize]
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

    fn clamp(&self, row: i32, col: i32) -> Option<(usize, usize)> {
        match (row, col) {
            i if i.0 < 0 => None,
            i if i.0 >= self.rows as i32 => None,
            i if i.1 < 0 => None,
            i if i.1 >= self.cols as i32 => None,
            _ => Some((row as usize, col as usize)),
        }
    }

    fn open_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col].state = CellState::Open;

        if self.cells[row][col].bomb_neighbours > 0 {
            return;
        }

        for row_offset in -1..2 {
            for col_offset in -1..2 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                match self.clamp(row as i32 + row_offset, col as i32 + col_offset) {
                    Some(c) => {
                        let cell = &mut self.cells[c.0][c.1];

                        match (&cell.state, &cell.cell_type, cell.bomb_neighbours) {
                            d if d.0 == &CellState::Open => continue,
                            d if d.1 == &CellType::Bomb => continue,
                            d if d.2 > 0 => cell.state = CellState::Open,
                            d if d.2 == 0 => {
                                cell.state = CellState::Open;
                                self.open_cell(c.0, c.1);
                            }
                            _ => continue,
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    fn mark_cell(&mut self, row: usize, col: usize) {
        match self.clamp(row as i32, col as i32) {
            Some(i) => match &mut self.cells[i.0][i.1] {
                c if c.state == CellState::Marked => c.state = CellState::Close,
                c if c.state == CellState::Close => c.state = CellState::Marked,
                _ => return,
            },
            _ => return,
        }
    }

    fn render(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = &self.cells[row][col];

                match cell.state {
                    CellState::Close => {
                        fill_rect(
                            col as i32 * self.cell_size + self.cell_border,
                            row as i32 * self.cell_size + self.cell_border,
                            self.cell_size - 2 * self.cell_border,
                            self.cell_size - 2 * self.cell_border,
                            100,
                            100,
                            100,
                            255,
                        );
                        continue;
                    }
                    CellState::Marked => {
                        fill_rect(
                            (col as i32 * self.cell_size) + (2 * self.cell_border),
                            (row as i32 * self.cell_size) + (2 * self.cell_border),
                            self.cell_size - (2 * 2 * self.cell_border),
                            self.cell_size - (2 * 2 * self.cell_border),
                            00,
                            100,
                            00,
                            255,
                        );
                        continue;
                    }

                    CellState::Open => match cell.cell_type {
                        CellType::Bomb => fill_rect(
                            col as i32 * self.cell_size + self.cell_border,
                            row as i32 * self.cell_size + self.cell_border,
                            self.cell_size - 2 * self.cell_border,
                            self.cell_size - 2 * self.cell_border,
                            200,
                            50,
                            50,
                            255,
                        ),
                        CellType::None => {
                            fill_rect(
                                col as i32 * self.cell_size + self.cell_border,
                                row as i32 * self.cell_size + self.cell_border,
                                self.cell_size - 2 * self.cell_border,
                                self.cell_size - 2 * self.cell_border,
                                200,
                                200,
                                200,
                                255,
                            );
                            if cell.bomb_neighbours > 0 {
                                fill_num(
                                    (col as i32 * self.cell_size) + (self.cell_size / 4),
                                    (row as i32 * self.cell_size) + (self.cell_size * 3 / 4),
                                    cell.bomb_neighbours,
                                );
                            }
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
                state: CellState::Close,
                bomb_neighbours: 0,
            };
            cols
        ];
        rows
    ];
    let mut game = Minesweeper {
        screen_height: screen_height,
        screen_width: screen_width,
        cell_size: cell_size,
        rows: rows,
        cols: cols,
        cells: cells,
        cell_border: 2,
        bomb_percent: 15,
        generated: false,
    };

    return Box::into_raw(Box::new(game));
}

#[no_mangle]
pub extern "C" fn render(game: *mut Minesweeper) {
    let game = unsafe { &mut *game };

    fill_rect(0, 0, game.screen_width, game.screen_height, 20, 20, 20, 255);

    game.render();
}

#[no_mangle]
pub extern "C" fn open_cell(game: *mut Minesweeper, row: usize, col: usize) {
    let game = unsafe { &mut *game };

    if !game.generated {
        game.generate_bombs(row, col);
    }

    game.open_cell(row, col);
}

#[no_mangle]
pub extern "C" fn mark_cell(game: *mut Minesweeper, row: usize, col: usize) {
    let game = unsafe { &mut *game };

    game.mark_cell(row, col);
}
