trait ITime {}

trait IRealmTime: ITime {
    fn get_time(&self) -> u32;
    fn set_time(&mut self, new_time: u32);
}

trait IObjectTime: ITime {
    fn get_offset_parent(&self) -> u32;
    fn set_offset_parent(&mut self, new_offset: u32);

    fn get_offset_local(&self) -> u32;
    fn set_offset_local(&mut self, new_offset: u32);

    fn get_scalar(&self) -> i32;
    fn set_scalar(&mut self, new_scalar: i32);

    fn get_interval(&self) -> u32;
    fn set_interval(&mut self, new_interval: u32);
}

pub struct RealmTime {
    time: u32,
}

pub struct ObjectTime {
    offset_parent: u32,
    offset_local: u32,
    scalar: i32,
    interval: u32,
}

impl ITime for RealmTime {}
impl ITime for ObjectTime {}

impl IRealmTime for RealmTime {
    fn get_time(&self) -> u32 {
        self.time
    }

    fn set_time(&mut self, new_time: u32) {
        self.time = new_time;
    }
}

impl IObjectTime for ObjectTime {
    fn get_offset_parent(&self) -> u32 {
        self.offset_parent
    }

    fn set_offset_parent(&mut self, new_offset: u32) {
        self.offset_parent = new_offset;
    }

    fn get_offset_local(&self) -> u32 {
        self.offset_local
    }

    fn set_offset_local(&mut self, new_offset: u32) {
        self.offset_local = new_offset;
    }

    fn get_scalar(&self) -> i32 {
        self.scalar
    }

    fn set_scalar(&mut self, new_scalar: i32) {
        self.scalar = new_scalar;
    }

    fn get_interval(&self) -> u32 {
        self.interval
    }

    fn set_interval(&mut self, new_interval: u32) {
        self.interval = new_interval;
    }
}
