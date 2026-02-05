pub(crate) fn main() {
    // Ensure internal logging can be enabled for debugging without changing user output.
    let filter = crate::util::env_filter();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init();
    let _ = tracing_log::LogTracer::init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Err(e) = super::run::run(&args) {
        if !e.is_silent() {
            eprintln!();
            eprintln!("✖ Error: {e}");
        }
        std::process::exit(1);
    }
}
