extern {
    #[link(name="my_lib_proj", kind="static")]
    fn hello() -> ();

    #[link(name="my_lib_proj", kind="static")]
    fn meaning_of_life() -> u32;

    #[link(name="my_lib_proj", kind="static")]
    fn die_die_die();
}

#[repr(C)] // "opaque type"
struct Calculator {
    _unused: [u8; 0],
}

extern {
    #[link(name="my_lib_proj", kind="static")]
    fn calculator_new() -> *mut Calculator;

    #[link(name="my_lib_proj", kind="static")]
    fn calculator_add(this: *mut Calculator, a: i32) -> i32;

    #[link(name="my_lib_proj", kind="static")]
    fn calculator_delete(this: *mut Calculator) -> ();
}

fn main() {
    unsafe {
        hello();
        println!("삶의 의미, 그것은 {}", meaning_of_life());

        let cal_ptr: *mut Calculator = calculator_new();
        println!("2 + 1 = {}", calculator_add(cal_ptr, 2));
        println!("3 + 1 = {}", calculator_add(cal_ptr, 3));
        calculator_delete(cal_ptr);

        die_die_die();
    }
}
