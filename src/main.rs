use cargo_metadata::{CrateType, Metadata, MetadataCommand, TargetKind};

fn main() -> anyhow::Result<()> {
    let Metadata {
        packages,
        workspace_members,
        ..
    } = MetadataCommand::new().exec()?;

    packages
        .into_iter()
        .filter(|package| {
            workspace_members.contains(&package.id)
                && package.targets.iter().any(|target| {
                    target
                        .crate_types
                        .iter()
                        .any(|crate_type| crate_type == &CrateType::Bin)
                        && target.kind.iter().any(|kind| kind == &TargetKind::Bin)
                })
        })
        .map(|package| package.name)
        .for_each(|name| {
            println!("{name}");
        });

    Ok(())
}
