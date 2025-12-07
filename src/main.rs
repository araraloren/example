use neure::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let array = regex::array(["a", "b", "c"]);

    // No match possible
    CharsCtx::with("xyz", move |mut ctx| {
        assert!(array.parse(&mut ctx));
        assert_eq!(ctx.offset(), 0);
    });
    Ok(())
}
