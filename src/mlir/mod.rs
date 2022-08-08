

#[derive(Debug)]
pub struct Operator {
    pub name: String,
    pub regions: Vec<Region>,
}

// #[derive(Debug)]
pub type Region = Vec<Block>;

#[derive(Debug)]
pub struct Block {
    pub label: Label,
    pub items: Vec<Operator>,
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
}









