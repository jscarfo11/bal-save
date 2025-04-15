#[derive(Debug, Clone)]
pub struct Filters {
    pub joker: String,
    pub misc: String,
    pub card: String,

    pub voucher: String,
}

impl Filters {
    pub fn new() -> Self {
        Filters {
            joker: String::new(),
            misc: String::new(),
            card: String::new(),
            voucher: String::new(),
        }
    }
}
