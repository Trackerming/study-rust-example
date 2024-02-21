use super::*;

#[test]
fn longer_str_with_announcement_test() {
    let str1 = String::from("short.");
    let str2 = String::from("longlong.");
    let anno = String::from("annoucement");
    let str_result = mix_usage::longer_str_with_announcement(&str1, &str2, &anno);
    assert_eq!(str2, str_result);
}

#[test]
fn not_equal_test() {
    assert_ne!(3, 3 + 2);
}

#[ignore]
#[test]
fn assert_test() {
    let name = "Jack";
    let name2 = "Tracking";
    let result = format!("hello {}", &name2);
    assert!(
        result.contains(&name),
        "test method did not contain name, value was {}",
        result
    );
}

// 包含关系？以下的也测试通过了
#[test]
#[should_panic(expected = "should panic msg")]
fn should_panic_test() {
    panic!("should panic msg.");
}

#[test]
fn result_generic_test() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("2 + 2 not equal 4"))
    }
}
