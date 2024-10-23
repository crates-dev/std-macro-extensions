use crate::*;
static STR: &str = "test";
static NUM: i32 = 1;

#[test]
fn test_arc_num() {
    let num_arc: Arc<i32> = arc!(NUM);
    let num: i32 = num_arc.as_ref().clone();
    assert_eq!(num_arc, Arc::new(NUM));
    assert_eq!(num, NUM)
}

#[test]
fn test_arc_str() {
    let str_arc: Arc<&str> = arc!(STR.clone());
    let tmp_str: &str = str_arc.as_ref().clone();
    assert_eq!(str_arc, Arc::new(STR.clone()));
    assert_eq!(tmp_str, STR.clone())
}

#[test]
fn test_arc_string() {
    let string_from_str: String = STR.to_string();
    let string_arc: Arc<String> = arc!(string_from_str.clone());
    let string: String = string_arc.as_ref().clone();
    assert_eq!(string_arc, Arc::new(string_from_str.clone()));
    assert_eq!(string, string_from_str.clone())
}
