fn main() {
    log(); 
}

fn log() {
    use web_sys::console;
    console::log_1(&"Hello using web-sys".into());
}