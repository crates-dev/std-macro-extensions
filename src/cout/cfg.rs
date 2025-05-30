#[test]
fn test_cout() {
    use crate::*;
    let name: &str = "Alice";
    let age: i32 = 30;
    cout!("Name: {}, Age: {}\n", name, age);
}

#[test]
fn test_endl() {
    use crate::*;
    endl!();
}

#[test]
fn test_cout_endl() {
    use crate::*;
    let name: &str = "Alice";
    let age: i32 = 30;
    cout_endl!("Name: {}, Age: {}\n", name, age);
}
