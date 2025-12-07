use neure::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let array = regex::array(["a", "b", "c"]);

    // No match possible
    CharsCtx::with("xyz", move |mut ctx| {
        array.try_parse(&mut ctx).unwrap();
    });
    Ok(())
}
