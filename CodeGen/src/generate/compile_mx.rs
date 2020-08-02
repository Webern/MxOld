use crate::error::Result;
use std::fs::{read_to_string, File};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use tempfile::TempDir;
use uuid::Uuid;

#[allow(unused)]
pub fn compile_mx<P: AsRef<Path>>(path_to_mx: P) -> Result<()> {
    let tempdir = wrap!(TempDir::new())?;
    let _ = run(
        Command::new("cmake")
            .arg(path_to_mx.as_ref())
            .arg("-DMX_BUILD_TESTS=on")
            .arg("-DMX_BUILD_EXAMPLES=on")
            .arg("-DMX_BUILD_CORE_TESTS=off"),
        tempdir.path(),
    )?;
    let _ = run(Command::new("make").arg("-j9"), tempdir.path())?;
    run_mx_tests(tempdir.path())?;
    Ok(())
}

#[allow(unused)]
fn run_mx_tests<P: AsRef<Path>>(path: P) -> Result<()> {
    let _ = run(&mut Command::new("./mxtest"), path.as_ref())?;
    Ok(())
}

#[allow(unused)]
fn run<P: AsRef<Path>>(cmd: &mut Command, dir: P) -> Result<(String, Output)> {
    let u = Uuid::new_v4();
    let opath = dir.as_ref().join(format!("combined_output.log.{}", u));
    let ofile = wrap!(File::create(&opath))?;
    let efile = wrap!(ofile.try_clone())?;
    let result = wrap!(cmd.current_dir(&dir.as_ref()).stdout(Stdio::from(ofile)).stderr(Stdio::from(efile)).spawn())?.wait_with_output();
    let output = wrap!(result)?;
    let outstr = wrap!(read_to_string(&opath))?;
    if !output.status.success() {
        return raise!("Command failed '{:?}':\n{}", &cmd, outstr);
    }
    println!("Command output '{:?}':\n{}", &cmd, outstr);
    Ok((outstr, output))
}
