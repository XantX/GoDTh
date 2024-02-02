#[derive(Debug, Clone)]
pub struct Kratos {
    pub weapon: Vec<String>,
    pub power: i32,
    pub habilities: Vec<String>,
    pub life: i32,
}

impl Kratos {
    pub fn new(weapon: Vec<String>, power: i32, habilities: Vec<String>, life: i32) -> Self { Self { weapon, power, habilities, life } }
}

