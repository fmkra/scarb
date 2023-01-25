use anyhow::{anyhow, Result};
use indicatif::HumanDuration;

use crate::compiler::targets::TargetCompilerMap;
use crate::compiler::CompilationUnit;
use crate::core::workspace::Workspace;
use crate::ops;
use crate::ui::Status;

#[tracing::instrument(skip_all, level = "debug")]
pub fn compile(ws: &Workspace<'_>) -> Result<()> {
    let resolve = ops::resolve_workspace(ws)?;
    let compilation_units = ops::generate_compilation_units(&resolve, ws)?;
    let mut compilers = TargetCompilerMap::new();

    // TODO(mkaput): Parallelize this loop.
    //   Caveat: This shouldn't be just rayon::map call, because we will introduce dependencies
    //   between compilation units in the future.
    for unit in compilation_units {
        compile_unit(unit, &mut compilers, ws)?;
    }

    let elapsed_time = HumanDuration(ws.config().elapsed_time());
    ws.config().ui().print(Status::new(
        "Finished",
        "green",
        &format!("release target(s) in {elapsed_time}"),
    ));

    Ok(())
}

fn compile_unit(
    unit: CompilationUnit,
    compilers: &mut TargetCompilerMap,
    ws: &Workspace<'_>,
) -> Result<()> {
    let package_name = unit.package.id.name.clone();

    ws.config()
        .ui()
        .print(Status::new("Compiling", "green", &unit.name()));

    compilers
        .load(&unit.target.kind)?
        .compile(unit, ws)
        .map_err(|err| {
            // TODO(mkaput): Make this an enum upstream.
            if err.to_string() == "Compilation failed." {
                anyhow!("could not compile `{package_name}` due to previous error")
            } else {
                err
            }
        })?;

    Ok(())
}
