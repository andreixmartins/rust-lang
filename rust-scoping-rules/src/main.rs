fn main() {

    // RAII (Resource Acquisition Is Initialization)
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);

        // _box3 is destroyed here
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // _box2 is destroyed here

}

fn create_box() {
    let _box1 = Box::new(3i32);
}

