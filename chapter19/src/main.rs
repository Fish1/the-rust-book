fn main() {
    println!("Hello, world!");

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 = {:?}", r1);
    println!("r2 = {:?}", r2);
        

    // we can use unsafe code blocks to de-reference
    // raw pointers
    unsafe {
        // unsafe rust allows us to mutate data
        // even though we have an immutable r1
        *r2 += 5;

        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);

        *r2 += 1;
        println!("*r2 = {}", *r2);
    }

    unsafe fn dangerous() {
        let mut x = 5;
        let r1 = &x as *const i32;
        let r2 = &mut x as *mut i32;
        unsafe {
            *r2 += 1;
            println!("r1 = {}", *r1);
        }
    }

    // dangerous();
    unsafe {
        dangerous();
    }
}
