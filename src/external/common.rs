use json::JsonValue;

#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub fn is_number_array(obj: &JsonValue, len: usize) -> bool {
    obj.is_array() && obj.len() == len && obj.members().fold(true, |m, x| m && x.is_number())
}

