pub struct Avocado {
    name: String,
    amount: i32,
}

impl Avocado {
    pub fn new(name: String, amount: i32) -> Self {
        Avocado { name, amount }
    }

    pub fn get_avocado(&self) -> &String {
        &self.name
    }

    pub fn get_amount(&self) -> &i32 {
        &self.amount
    }

    pub fn eat_avocado(&mut self) {
        self.amount -= 1;
        println!("eating avocado: {}, left: {}", self.get_avocado(), self.get_amount());
    }
}
