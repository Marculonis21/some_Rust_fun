struct MyStruct<T> {
    item: T, 
}

impl<T: Clone> MyStruct<T> {
}

// We all know how big it is...


// object safety
// unsafe
trait NotSafe<T> {
    fn generic_method<U>(x: T, y: U);
    fn uses_self(self: Self) -> Self;
}

// safe
trait Safe {
    fn concrete_method(&self, x: i32);
    fn returns_self(&self) -> Box<dyn Safe>;
}

fn main() {
    println!("Hello, world!");
}
