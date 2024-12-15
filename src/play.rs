pub fn hello() {
    println!("{}", "hello play branch");
    println!("{}", "2024/12/15");
}

mod food {
    #[derive(Debug)]
    pub struct Cake {
        pub cake_name: String,
    }

    impl Cake {
        pub fn bake(&self) -> String {
            let action = "baking";
            let msg = format!("{} {}", action, self.cake_name);
            println!("=> {}", msg);
            msg
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn food_works() {
        let c = food::Cake {
            cake_name: "yummy cake".to_string(),
        };
        println!("{:?}", c);
        let msg = c.bake();
        assert_eq!(msg, format!("baking {}", c.cake_name));
    }
}
