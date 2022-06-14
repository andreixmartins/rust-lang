fn main() {

    // MUTABILITY
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line


    // SCOPE
    let scope1 = 1;
    {
        let scope2 = 1;
        println!("scope2: {}", scope2);
    }
    // println!("scope2: {}", scope2); //Error
    println!("scope1: {}", scope1);


    // SHADOWING
    let shadow = 1;
    {
        let shadow = "test";
        println!("shadow: {}",shadow)
    }
    println!("outside shadow variable: {}",shadow);

    let shadow = 2;
    println!("outside shadow variable 2: {}",shadow);



    // FREEZING
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50; //error
        // FIXME ^ Comment out this line
    }
    _mutable_integer = 3;
}
