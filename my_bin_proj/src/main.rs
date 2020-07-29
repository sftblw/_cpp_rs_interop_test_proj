extern {
    #[link(name="my_lib_proj", kind="static")]
    fn hello() -> ();

    #[link(name="my_lib_proj", kind="static")]
    fn meaning_of_life() -> u32;

    #[link(name="my_lib_proj", kind="static")]
    fn die_die_die();
}

fn main() {
    unsafe {
        hello();
        println!("삶의 의미, 그것은 {}", meaning_of_life());
        die_die_die();
    }
}
