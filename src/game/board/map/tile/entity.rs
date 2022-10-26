use super::Tile;
use crate::constant::*;
use crate::error::CtrlErr;

#[derive(Clone)]
#[derive(Debug)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

impl Terrian {
    pub fn str(&self) -> String {
        match self {
            Terrian::Plain => "Plain".to_string(),
            Terrian::Sea => "Sea".to_string(),
            Terrian::Hill => "Hill".to_string(),
        }
    }
    
    pub(super) fn mvcost(&self) -> Result<f64, CtrlErr> {
        match self {
            Terrian::Plain => Ok(MVCOST_PLAIN),
            Terrian::Hill => Ok(MVCOST_HILL),
            Terrian::Sea => Err(CtrlErr::WrongTerrian(Terrian::Sea)),
        }
    }

    pub(super) fn can_step(&self) -> Result<(), CtrlErr> {
        match self {
            Terrian::Sea => Err(CtrlErr::WrongTerrian(Terrian::Sea)),
            _ => Ok(()),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), CtrlErr> {
        match self {
            Terrian::Sea => Err(CtrlErr::WrongTerrian(Terrian::Sea)),
            _ => Ok(()),
        }
    }

    pub(super) fn can_sow(&self) -> Result<(), CtrlErr> {
        match self {
            Terrian::Sea => Err(CtrlErr::WrongTerrian(Terrian::Sea)),
            _ => Ok(()),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Natural {
    Tree,
    Farm,
}

impl Natural {
    pub fn str(&self) -> String {
        match self {
            Natural::Tree => "Tree".to_string(),
            Natural::Farm => "Farm".to_string(),
        }
    }
    
    pub(super) fn mvcost(&self) -> Result<f64, CtrlErr> {
        match self {
            Natural::Tree => Ok(MVCOST_TREE),
            _ => Ok(0.),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Resource {
    Food,
    Wood,
}

impl Resource {
    pub fn str(&self) -> String {
        match self {
            Resource::Food => "Food".to_string(),
            Resource::Wood => "Wood".to_string(),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Manmade {
    Hovel,
    Sawmill,
}

impl Manmade {
    pub fn str(&self) -> String {
        match self {
            Manmade::Hovel => "Hovel".to_string(),
            Manmade::Sawmill => "Sawmill".to_string(),
        }
    }
    
    pub(super) fn max_process(&self) -> i64 {
        match self {
            Manmade::Hovel => PROCESS_HOVEL,
            Manmade::Sawmill => PROCESS_SAWMILL,
            _ => 0,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}

impl Placement {
    pub fn str(&self) -> String {
        match self {
            Placement::Void => "Nothing".to_string(),
            Placement::Landform(n) => format!("{} as Landform", n.str()),
            Placement::Building(m) => format!("{} as Building", m.str()),
            Placement::Foundation(m, p) => format!("{} as Foundation", m.str()),
        }
    }
    
    pub(super) fn is_hovel(&self) -> bool {
        matches!(self, Placement::Building(Manmade::Hovel))
    }

    pub(super) fn is_sawmill(&self) -> bool {
        matches!(self, Placement::Building(Manmade::Sawmill))
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

    pub(super) fn mvcost(&self) -> Result<f64, CtrlErr> {
        match self {
            Placement::Landform(n) => n.mvcost(),
            _ => Ok(0.),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), CtrlErr> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(natural) => match natural {
                Natural::Tree => Err(CtrlErr::WrongPlacement(self.clone())),
                Natural::Farm => Ok(()),
            }
            other => Err(CtrlErr::WrongPlacement(self.clone())),
        }
    }

    pub(super) fn found(&mut self, manmade: Manmade) -> Result<(), CtrlErr> {
        self.can_found();
        *self = Placement::Foundation(manmade, 0);
        Ok(())
    }

    pub(super) fn can_sow(&self) -> Result<(), CtrlErr> {
        match self {
            Placement::Void => Ok(()),
            other => Err(CtrlErr::WrongPlacement(self.clone()))
        }
    }

    pub fn sow(&mut self) -> Result<(), CtrlErr> {
        self.can_sow()?;
        *self = Placement::Landform(Natural::Farm);
        Ok(())
    }

    pub(super) fn can_build(&self) -> Result<(), CtrlErr> {
        if let Placement::Foundation(..) = self {
            Ok(())
        }else{
            Err(CtrlErr::WrongPlacement(self.clone()))
        }
    }

    pub(super) fn build(&mut self) -> Result<bool, CtrlErr> {
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
