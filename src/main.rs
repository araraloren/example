use neure::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = '9'.at_least::<1>();
    let ver = num.sep_once(".", num);

    let mut ctx = CharsCtx::new("");

    ctx.ctor(&ver)?;

    let num = '9'.at_most::<1>();
    let ver = num.sep_once(".", num);

    let mut ctx = CharsCtx::new("");

    ctx.ctor(&ver)?;

    let num = '9'.between::<1, 2>();
    let ver = num.sep_once(".", num);

    let mut ctx = CharsCtx::new("");

    ctx.ctor(&ver)?;

    Ok(())
}
