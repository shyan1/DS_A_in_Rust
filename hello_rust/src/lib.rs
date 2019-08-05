pub struct Door {
    is_open: bool,
}

#[allow(dead_code)]
impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open: is_open }
    }
}

pub trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_door() {
        use super::*;
        let mut door = Door::new(false);
        door.open();

        assert!(door.is_open);
    }
}
