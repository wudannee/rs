pub fn run() {
    use crate::avocado::Avocado;
    println!("==== hello play ====");
    let mut avocado = Avocado::new("avocado-1".to_string(), 10);
    loop {
       if let Err(err_msg) = avocado.eat_avocado() {
           println!("{}", err_msg);
           break;
       }
    }
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
