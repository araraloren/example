fn main() -> color_eyre::Result<()> {
    wit_deps::lock_sync!()
        .map_err(|e| color_eyre::eyre::eyre!("can not sync dependencies of snippet-c: {e:?}"))?;
    Ok(())
}
