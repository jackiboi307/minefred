// pub struct ZIndex {
//     pub index: u8,
// }

pub enum ZIndex {
    HIGH,
}

impl ZIndex {
    pub fn new(index: u8) -> Self {
        // Self{
            // index
        // }

        match index {
            1 => Self::HIGH,
            _ => { panic!("Invalid value for ZIndex: {}", index); }
        }
    }
}
