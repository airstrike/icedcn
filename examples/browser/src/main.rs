// rawr

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    if std::env::args().nth(1).as_deref() == Some("--export-web-routes") {
        use browser::app::web;
        use std::process;
        if let Err(error) = web::write_routes() {
            eprintln!("Could not export web routes: {error}");
            process::exit(1);
        }
        return Ok(());
    }
    browser::run()
}
