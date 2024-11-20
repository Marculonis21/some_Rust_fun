trait JS {
    fn truthful(&self) -> bool;
}

impl JS for u8 {
    fn truthful(&self) -> bool {
        *self != 0
    }
}

impl JS for String {
    fn truthful(&self) -> bool {
        !self.trim().is_empty()
    }
}

impl PartialEq for dyn JS {
    fn eq(&self, other: &Self) -> bool {
        self.truthful() == other.truthful()
    }
}

// impl Deref for dyn JS {
//     type Target = bool;

//     fn deref(&self) -> &Self::Target {
//         &self.truthful()
//     }
// }

fn main() {
    println!("Hello, world!");

    println!("{:?}", String::from("Ahoj").truthful());
    println!("{:?}", String::from("").truthful());
    println!("{:?}", String::from(" ").truthful());

    println!("{:?}", 8.truthful());
    println!("{:?}", 0.truthful());
    
    println!("{:?}", (&8u8 as &dyn JS) == (&1u8 as &dyn JS));

}
