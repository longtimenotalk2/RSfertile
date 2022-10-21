

use crate::constant::*;
use super::Tile;

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

#[derive(Clone)]
pub enum Landform {
    Void,
    Tree,
}

#[derive(Clone)]
pub enum Building {
    Void,
    Foundation(Box<(Building, i64)>),
    Farm,
    Hovel,
}

impl Building {
    pub fn remained_mount(&self) -> i64 {
        match self {
            Building::Foundation(f) => match f.0 {
                Building::Hovel => MOUNT_HOVEL - f.1, 
                _ => 0,
            }
            _ => 0,
        }
    }

    pub(super) fn build(&mut self, mount : i64) -> bool {
        let diff = mount - self.remained_mount();
        match self {
            Building::Foundation(f) => {
                match diff {
                    diff if diff > 0 => panic!("mount over"),
                    diff if diff == 0 => {
                        *self = f.0.clone();
                        true
                    },
                    _ => {
                        *self = Building::Foundation(Box::new((f.0.clone(), f.1 + mount)));
                        false
                    },
                }
            },
            _ => panic!("Can not build on not foundation")
        }
    }
}

        


