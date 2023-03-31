struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: &str) -> Self {
        CustomSmartPointer {
            data: data.to_string(),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
        drop(&self.data);
    }
}

fn main() {
    let _a = CustomSmartPointer::new("pointer A");
    let _b = CustomSmartPointer::new("pointer B");

    println!("Hello, world!");
}
