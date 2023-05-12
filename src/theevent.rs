//use crate::prelude::*;

pub struct TheEventUpdate {
    /// Current time
    pub time            : u128,
    /// Window needs a single refresh
    pub single          : bool,
    /// Window needs refreshes at 60fps until the given time
    pub until           : u128,
    /// The current mouse position
    pub mouse_pos       : Option<(u32, u32)>
}

impl TheEventUpdate {

    pub fn new() -> Self {
        Self {
            time        : 0,
            single      : false,
            until       : 0,
            mouse_pos   : None
        }
    }

    pub fn single() -> Self {
        Self {
            time        : 0,
            single      : true,
            until       : 0,
            mouse_pos   : None
        }
    }

    pub fn until(until: u128) -> Self {
        Self {
            time        : 0,
            single      : false,
            until,
            mouse_pos   : None
        }
    }

    pub fn integrate_until(&mut self, until: u128) {
        if until > self.until {
            self.until = until;
        }
    }

}