use std::{env, process::Command};

use anyhow::Result;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => build()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

build            builds all languages 
"
    )
}

fn build() -> Result<()> {
    run_libsql_sqlite3("./configure")?;
    run_libsql_sqlite3("make")?;

    Ok(())
}

fn run_libsql_sqlite3(cmd: &str) -> Result<()> {
    let mut out = Command::new(cmd).current_dir("libsql-sqlite3").spawn()?;

    let exit = out.wait()?;

    if !exit.success() {
        anyhow::bail!("non 0 exit code: {}", exit);
    }

    Ok(())
}
