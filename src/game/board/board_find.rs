use super::Board;

pub enum Dir{
    R,
    DR,
    DL,
    L,
    UL,
    UR,
}

#[derive(Clone)]
pub struct Pos {
    r : i64,
    c : i64,
}

impl Pos {
    pub fn new(r : i64, c : i64) -> Self {
        Self {
            r,
            c,
        }
    }

    pub fn into_usize(&self, n_col: i64) -> usize {
        let i = self.r * n_col + self.c;
        i.try_into().unwrap()
    }

    pub fn eq(&self, pos : &Pos) -> bool{
        self.r == pos.r && self.c == pos.c 
    }

    pub fn get(&self) -> (i64, i64) {
        (self.r, self.c)
    }
}

impl Board {
    pub fn valid(&self, pos : &Pos) -> bool {
        let (r, c) = (pos.r, pos.c);
        if r >= 0 && r < self.n_row && c >= 0 && c < self.n_col {true} else {false}
    }

    pub fn find_all(&self) -> Vec<Pos> {
        let mut result : Vec<Pos> = vec!();
        for row in 0..self.n_row {
            for col in 0..self.n_col {
                result.push(Pos::new(row, col));
            }
        }
        result
    }
    
    pub fn find_dir(&self, pos : &Pos, dir : &Dir) -> Option<Pos> {
      if self.valid(pos){
          let (row, col) = (pos.r, pos.c);
        match dir {
            Dir::R => {
                if col < self.n_col - 1 {Some(Pos::new(row, col+1))}else{None}
            },
            Dir::DR => {
                if row < self.n_row - 1 && (row % 2 == 0 || col < self.n_col - 1) {Some(Pos::new(row+1, col+row%2))} else {None}
            },
            Dir::DL => {
                if row < self.n_row - 1 && (row % 2 == 1 || col > 0) {Some(Pos::new(row+1, col-(1-row%2)))} else {None}
            },
            Dir::L => {
                if col > 0 {Some(Pos::new(row, col-1))} else {None}
            },
            Dir::UL => {
                if row > 0 && (row % 2 == 1 || col > 0) {Some(Pos::new(row-1, col-(1-row%2)))} else {None}
            },
            Dir::UR => {
                if row > 0 && (row % 2 == 0 || col < self.n_col - 1) {Some(Pos::new(row-1, col+row%2))} else {None}
            },
        }
      }else{None}
    }

    pub fn find_adjs(&self, pos : &Pos) -> Vec<Pos> {
        if self.valid(pos) {
           [Dir::R, Dir::DR, Dir::DL, Dir::L, Dir::UL, Dir::UR].iter().filter_map(|d| self.find_dir(pos, d)).collect()
        }else{
            vec!()
        }
    }
    
    pub(super) fn num_adjs(&self, pos : &Pos) -> u32 {
        self.find_adjs(pos).len().try_into().unwrap()
    }
}