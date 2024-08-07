
mod gui;

#[tokio::main]
async fn main() {
    unsafe {
        let _ = libloading::Library::new("target/debug/libdyn_lib.so").unwrap();
    }
    gui::run();
}
