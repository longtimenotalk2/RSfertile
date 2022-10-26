use super::board::map::map_find::Dir;
use super::board::map::tile::entity::Manmade;
use super::Game;
use crate::error::CtrlErr;

impl Game {
    fn update(&mut self) {
        let board_new = self.board().clone();
        self.boards.push(board_new);
    }

    pub(super) fn cmd_undo(&mut self) -> Result<(), CtrlErr> {
        if self.boards.len() > 1 {
            self.boards.pop();
            self.show();
            Ok(())
        } else {
            Err(CtrlErr::Undo)
        }
    }

    pub(super) fn cmd_move(&mut self, dir: &Dir) -> Result<(), CtrlErr> {
        self.board().king_can_move(dir)?;
        self.update();
        self.board_mut().king_move(dir).expect("panic in king move");
        self.show();
        Ok(())
    }

    pub(super) fn cmd_pick(&mut self) -> Result<(), CtrlErr> {
        self.board().king_can_pick()?;
        self.update();
        self.board_mut().king_pick().expect("panic in king puck");
        self.show();
        Ok(())
    }

    pub(super) fn cmd_found(&mut self, manmade: Manmade)  -> Result<(), CtrlErr> {
        self.board().king_can_found()?;
        self.update();
        self.board_mut()
            .king_found(manmade)
            .expect("panic in king found");
        self.show();
        Ok(())
    }

    pub(super) fn cmd_build(&mut self)  -> Result<(), CtrlErr> {
        self.board().king_can_build()?;
        self.update();
        self.board_mut().king_build().expect("panic in king build");
        self.show();
        Ok(())
    }

    pub(super) fn cmd_saw(&mut self)  -> Result<(), CtrlErr> {
        self.board().king_can_saw()?;
        self.update();
        self.board_mut().king_saw().expect("panic in king saw");
        self.show();
        Ok(())
    }

    pub(super) fn cmd_end(&mut self) -> Result<(), CtrlErr> {
        self.update();
        self.board_mut().king_end();
        self.show();
        Ok(())
    }

    pub(super) fn cmd_invalid(&self) -> Result<(), CtrlErr> {
        Err(CtrlErr::Input)
    }
}
