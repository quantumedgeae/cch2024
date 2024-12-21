use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::fmt::Write;
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

const EMTY: &str = "⬛";
const COKY: &str = "🍪";
const MILK: &str = "🥛";
const WALL: &str = "⬜";

const ROWS: usize = 5;
const COLS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardTile {
    EMPTY,
    COOKIE,
    MILK,
    WALL,
}
impl Display for BoardTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EMPTY => write!(f, "{}", EMTY),
            Self::COOKIE => write!(f, "{}", COKY),
            Self::MILK => write!(f, "{}", MILK),
            Self::WALL => write!(f, "{}", WALL),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BoardState {
    grid: [[BoardTile; COLS]; ROWS],
    smap: HashMap<usize, u8>,
    info: Option<String>,
    seed: StdRng,
}
impl BoardState {
    pub fn rwlocked_default() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::default()))
    }
    fn reset(&mut self) {
        let grid = &mut self.grid;
        let mut grid_values = [[BoardTile::EMPTY; COLS]; ROWS];
        std::mem::swap(grid, &mut grid_values);
        (0..ROWS).for_each(|r| {
            grid[r][0] = BoardTile::WALL;
            grid[r][COLS - 1] = BoardTile::WALL
        });
        (0..COLS).for_each(|c| grid[ROWS - 1][c] = BoardTile::WALL);

        let col_vals = &mut self.smap;
        (0..COLS).for_each(|c| {
            col_vals.insert(c, 4 as u8);
        });

        self.info = None;

        self.seed = StdRng::seed_from_u64(2024);
    }
    fn randomize(&mut self) {
        let board_grid = &mut self.grid;
        (0..ROWS - 1).for_each(|r| {
            (1..COLS - 1).for_each(|c| {
                if self.seed.gen::<bool>() {
                    board_grid[r][c] = BoardTile::COOKIE;
                } else {
                    board_grid[r][c] = BoardTile::MILK;
                }
            });
        });

        let col_vals = &mut self.smap;
        (0..COLS).for_each(|c| {
            col_vals.insert(c, 0 as u8);
        });
        self.check_winner()
    }
    fn check_winner(&mut self) {
        let mut others_count = 0;

        for r in 0..ROWS - 1 {
            let mut c_count = 0;
            let mut c_r_count = 0;

            let mut m_count = 0;
            let mut m_r_count = 0;
            for c in 1..COLS - 1 {
                if BoardTile::COOKIE == self.grid[r][c] {
                    c_count += 1;
                } else if BoardTile::MILK == self.grid[r][c] {
                    m_count += 1;
                } else {
                    others_count += 1;
                }

                if BoardTile::COOKIE == self.grid[c - 1][r + 1] {
                    c_r_count += 1;
                } else if BoardTile::MILK == self.grid[c - 1][r + 1] {
                    m_r_count += 1;
                }
            }

            if 4 == c_count || 4 == c_r_count {
                self.info = Some(format!("{} wins!", COKY));
                return;
            }

            if 4 == m_count || 4 == m_r_count {
                self.info = Some(format!("{} wins!", MILK));
                return;
            }
        }
        if 0 == others_count {
            self.info = Some(format!("No winner."));
        }

        let mut c_d_count = 0;
        let mut c_d_r_count = 0;
        let mut m_d_count = 0;
        let mut m_d_r_count = 0;
        for r in 0..ROWS - 1 {
            if BoardTile::COOKIE == self.grid[r][r + 1] {
                c_d_count += 1;
            } else if BoardTile::MILK == self.grid[r][r + 1] {
                m_d_count += 1;
            }
            if BoardTile::COOKIE == self.grid[r][4 - r] {
                c_d_r_count += 1;
            } else if BoardTile::MILK == self.grid[r][4 - r] {
                m_d_r_count += 1;
            }
        }
        if 4 == c_d_count || 4 == c_d_r_count {
            self.info = Some(format!("{} wins!", COKY));
        }
        if 4 == m_d_count || 4 == m_d_r_count {
            self.info = Some(format!("{} wins!", MILK));
        }
    }
}
impl Default for BoardState {
    fn default() -> Self {
        let mut new_board_state = Self {
            grid: [[BoardTile::EMPTY; COLS]; ROWS],
            smap: HashMap::new(),
            info: None,
            seed: StdRng::seed_from_u64(2024),
        };
        new_board_state.reset();
        new_board_state
    }
}
impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = String::new();
        for r in 0..ROWS {
            for c in 0..COLS {
                write!(&mut b, "{}", self.grid[r][c])?;
            }
            write!(&mut b, "\n")?;
        }
        if let Some(info) = &self.info {
            write!(&mut b, "{}\n", info)?;
        }
        write!(f, "{}", b)
    }
}

pub async fn show_board(Extension(state): Extension<Arc<RwLock<BoardState>>>) -> impl IntoResponse {
    (StatusCode::OK, state.read().unwrap().to_string())
}

pub async fn reset_board(
    Extension(state): Extension<Arc<RwLock<BoardState>>>,
) -> impl IntoResponse {
    let mut board_state = state.write().unwrap();
    board_state.reset();
    StatusCode::OK
}

pub async fn randomize_board(
    Extension(state): Extension<Arc<RwLock<BoardState>>>,
) -> impl IntoResponse {
    let mut board_state = state.write().unwrap();
    board_state.randomize();
    (StatusCode::OK, board_state.to_string())
}

const TEAM_COKY: &str = "cookie";
const TEAM_MILK: &str = "milk";

pub async fn place(
    Path((team, column)): Path<(String, u8)>,
    Extension(state): Extension<Arc<RwLock<BoardState>>>,
) -> impl IntoResponse {
    let ref team_str = team.as_str();
    if ![TEAM_COKY, TEAM_MILK].contains(team_str) {
        return (StatusCode::BAD_REQUEST, "".to_owned());
    }
    if 1 > column || column > 4 {
        return (StatusCode::BAD_REQUEST, "".to_owned());
    }
    let mut board_state = state.write().unwrap();
    let smap_col_next_val: usize = board_state.smap[&(column as usize)] as usize;
    if smap_col_next_val == 0 {
        return (StatusCode::SERVICE_UNAVAILABLE, "".to_owned());
    }
    if let Some(_) = &board_state.info {
        return (StatusCode::SERVICE_UNAVAILABLE, "".to_owned());
    }
    let board_grid = &mut board_state.grid;
    if TEAM_COKY == *team_str {
        board_grid[smap_col_next_val - 1][column as usize] = BoardTile::COOKIE;
    } else if TEAM_MILK == *team_str {
        board_grid[smap_col_next_val - 1][column as usize] = BoardTile::MILK;
    }
    board_state
        .smap
        .insert(column as usize, (smap_col_next_val - 1) as u8);
    board_state.check_winner();
    (StatusCode::OK, board_state.to_string())
}
