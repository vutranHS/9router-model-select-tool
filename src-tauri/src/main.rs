fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(code) = nine_router_model_selector_lib::run_capability_cli(&args) {
        std::process::exit(code);
    }
    nine_router_model_selector_lib::run();
}
