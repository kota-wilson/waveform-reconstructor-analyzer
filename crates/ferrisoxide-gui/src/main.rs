#[cfg(feature = "native")]
mod native;

#[cfg(feature = "native")]
fn main() -> eframe::Result<()> {
    native::run()
}

#[cfg(not(feature = "native"))]
fn main() {
    eprintln!("ferrisoxide-gui requires --features native to launch the egui app");
    std::process::exit(2);
}
