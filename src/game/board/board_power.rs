use super::Board;
use crate::error::CtrlErr;

#[derive(Clone)]
pub struct ManPower {
    power_total : i64,
}

impl ManPower {
    pub fn new() -> Self {
        Self { power_total: 0 }
    }
    
    pub fn enough(&self, power : i64) -> Result<(), CtrlErr> {
        if power <= self.power_total {
            Ok(())
        }else{
            Err(CtrlErr::LackPower(power, self.power_total))
        }
    }

    pub fn inject(&mut self, power : i64) {
        self.power_total += power;
    }

    pub fn employ(&mut self, power : i64) -> Result<(), CtrlErr> {
        self.enough(power)?;
        self.power_total -= power;
        Ok(())
    }
}