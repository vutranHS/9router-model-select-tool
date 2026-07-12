use nine_router_model_selector_lib::cli_setup;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) != Some("setup") {
        eprintln!("Usage: 9router-model-selector setup [--model <model-id>] [--token <token>]");
        std::process::exit(2);
    }
    let combo = args.windows(2).find(|pair| pair[0] == "--model").map(|pair| pair[1].clone()).unwrap_or_else(|| "cc/claude-sonnet-5".into());
    let token = args.windows(2).find(|pair| pair[0] == "--token").map(|pair| pair[1].clone()).or_else(|| std::env::var("NINE_ROUTER_TOKEN").ok()).unwrap_or_else(|| { eprintln!("A token is required: use --token or NINE_ROUTER_TOKEN."); std::process::exit(2) });
    match cli_setup(combo, token) {
        Ok(changes) => { for change in changes { println!("✓ {change}"); } }
        Err(error) => { eprintln!("Setup failed: {error}"); std::process::exit(1); }
    }
}
