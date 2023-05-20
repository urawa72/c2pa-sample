use c2pa::{assertions::Actions, ManifestStore, Result};

fn main() {
    let result =execute();
    println!("{:?}", result);
}

fn execute() -> Result<()> {
    let manifest_store = ManifestStore::from_file("tests/fixtures/test_file.jpg")?;
    println!("{}", manifest_store);

    if let Some(manifest) = manifest_store.get_active() {
        let actions: Actions = manifest.find_assertion(Actions::LABEL)?;
        for action in actions.actions {
            println!("{}\n", action.action());
        }
    }

    Ok(())
}
