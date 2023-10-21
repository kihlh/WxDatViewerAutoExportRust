pub trait LoadConfigValue {
    fn to_bool(&self) -> bool;
    fn to_usize(&self) -> usize;
    fn to_i32(&self) -> i32;
}

impl LoadConfigValue for bool {
    fn to_bool(&self) -> bool {
        self.clone()
    }

    fn to_usize(&self) -> usize {
        0
    }

    fn to_i32(&self) -> i32 {
       0
    }
}

impl LoadConfigValue for usize {
    fn to_bool(&self) -> bool {
        self.eq(&1usize)
    }

    fn to_usize(&self) -> usize {
        self.clone()
    }

    fn to_i32(&self) -> i32 {
        self.clone() as i32
    }
}

impl LoadConfigValue for i32 {
    fn to_bool(&self) -> bool {
        self.eq(&1i32)
    }

    fn to_usize(&self) -> usize {
        self.clone() as usize
    }

    fn to_i32(&self) -> i32 {
        self.clone()
    }
}
