use std::env;
use std::sync::Mutex;
use std::process::Command;

// Tests are run in alphabetical order, and concurrently

static TEST_MUTEX : Mutex<i32> = Mutex::new(1);

#[test]
fn test_0_setvalue(){
    let _guard = TEST_MUTEX.lock().unwrap();
    println!("test_0_setvalue RUN");

    env::set_var("TEST_KEY", "value");

    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    std::env::set_var("TEST_KEY", "");

    println!("test_0_setvalue END");
}

#[test]
fn test_1_readvalue(){
    let _guard = TEST_MUTEX.lock().unwrap();
    println!("test_1_readvalue RUN");

    let value = match std::env::var_os("TEST_KEY"){
        Some(x) => x.into_string().unwrap(),
        None => String::from("None")
    };
    assert_eq!(String::from(""), value);
    println!("test_1_readvalue END");
}
