use super::Tile;
use crate::constant::*;

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

impl Terrian {
    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Terrian::Plain => Ok(MVCOST_PLAIN),
            Terrian::Hill => Ok(MVCOST_HILL),
            Terrian::Sea => Err("Can not get Sea's mvcost"),
        }
    }

    pub(super) fn can_step(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not step on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not found on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub(super) fn can_sow(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not sow on the Sea tile."),
            _ => Ok(()),
        }
    }
}

#[derive(Clone)]
pub enum Natural {
    Tree,
    Farm,
}

impl Natural {
    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Natural::Tree => Ok(MVCOST_TREE),
            _ => Ok(0.),
        }
    }
}

#[derive(Clone)]
pub enum Resource {
    Food,
    Wood,
}

#[derive(Clone)]
pub enum Manmade {
    Hovel,
    Sawmill,
}

impl Manmade {
    pub(super) fn max_process(&self) -> i64 {
        match self {
            Manmade::Hovel => PROCESS_HOVEL,
            Manmade::Sawmill => PROCESS_SAWMILL,
            _ => 0,
        }
    }
}

#[derive(Clone)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}

impl Placement {
    pub(super) fn is_hovel(&self) -> bool {
        if let Placement::Building(m) = self {
            if let Manmade::Hovel = m {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn remained_process(&self) -> Option<i64> {
        if let Placement::Foundation(m, p) = self {
            if *p < 0 {
                panic!("Should not has minus process")
            }
            let rp = m.max_process() - p;
            if rp < 0 {
                panic!("Should not get minus remained process")
            }
            Some(rp)
        }else{
            None
        }
    }

    pub fn get_process(&self) -> Option<i64> {
        if let Placement::Foundation(m, p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Placement::Landform(n) => n.mvcost(),
            _ => Ok(0.),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), &'static str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(natural) => match natural {
                Natural::Tree => Err("Can not found on a tile with Tree."),
                Natural::Farm => Ok(()),
            },
            Placement::Building(_) => Err("Can not found on a Building."),
            Placement::Foundation(..) => Err("Can not found on a Foundation."),
        }
    }

    pub(super) fn found(&mut self, manmade: Manmade) -> Result<(), &'static str> {
        self.can_found();
        *self = Placement::Foundation(manmade, 0);
        Ok(())
    }

    pub(super) fn can_sow(&self) -> Result<(), &'static str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(_) => Err("Can sow sow on a tile with Natural"),
            Placement::Building(_) => Err("Can not sow on a Building."),
            Placement::Foundation(..) => Err("Can not sow on a Foundation."),
        }
    }

    pub fn sow(&mut self) -> Result<(), &'static str> {
        self.can_sow()?;
        *self = Placement::Landform(Natural::Farm);
        Ok(())
    }

    pub(super) fn can_build(&self) -> Result<(), &'static str> {
        if let Placement::Foundation(..) = self {
            Ok(())
        }else{
            Err("Can only build on Foundation")
        }
    }

    pub(super) fn build(&mut self) -> Result<bool, &'static str> {
        self.can_build()?;
        if let Placement::Foundation(m, p) = self {
            *p += 1;
        }
        if self.remained_process().unwrap() == 0 {
            if let Placement::Foundation(m, p) = self {
                *self = Placement::Building(m.clone());
                Ok(true)
            }else{
                panic!("")
            }
        }else{
            Ok(false)
        }
    }

    pub(super) fn produce(&self) -> Option<Resource> {
        match self {
            Placement::Landform(n) => match n {
                Natural::Farm => Some(Resource::Food),
                Natural::Tree => Some(Resource::Wood),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn can_produce_food(&self) -> bool {
        if let Some(r) = self.produce() {
            if let Resource::Food = r {
                true
            }else{
                false
            }
        }else{
            false
        }
    }

}
