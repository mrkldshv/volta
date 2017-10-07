extern crate reqwest;
extern crate tar;
extern crate flate2;
extern crate toml;
extern crate term_size;
extern crate indicatif;

use std::convert::AsRef;
use std::env;
use std::env::ArgsOs;
use std::ffi::{OsString, OsStr};
use std::path::PathBuf;
use std::iter::Extend;
use std::process::Command;

#[cfg(windows)]
extern crate winfolder;

mod config;
mod provision;
mod install;

use config::{Config, Version};

/**
 * Produces a pair containing the executable name (as passed in the first
 * element of `argv`) and the command-line arguments (as found in the rest
 * of `argv`).
 */
fn command_and_args() -> Option<(OsString, ArgsOs)> {
    let mut args = env::args_os();
    args.next().map(|arg0| { (arg0, args) })
}

/**
 * Produce a modified version of the current `PATH` environment varible that
 * will find Node.js executables in the installation directory for the given
 * version of Node instead of in the nodeup binstubs directory.
 */
fn instantiate_path<T: AsRef<OsStr>>(current: &T, version: &str) -> OsString {
    let nodeup_bin = &config::nodeup_binstubs().unwrap();
    let split = env::split_paths(current).filter(|s| { s != nodeup_bin });
    let mut path_vec: Vec<PathBuf> = Vec::new();
    path_vec.push(config::node_version_root(version).map(|root| root.join("bin")).unwrap());
    path_vec.extend(split);
    env::join_paths(path_vec.iter()).unwrap()
}

fn main() {
    println!("{:?}", winfolder::known_path(&winfolder::id::LOCAL_APP_DATA));
    println!("{:?}", winfolder::known_path(&winfolder::id::PROGRAM_DATA));
    println!("{:?}", winfolder::known_path(&winfolder::id::PROGRAM_FILES));
    println!("{:?}", winfolder::known_path(&winfolder::id::PROGRAM_FILES_X64));
    println!("{:?}", winfolder::known_path(&winfolder::id::PROGRAM_FILES_X86));

    panic!("poopnuggets");

    // FIXME: handle None
    let Config { node: Version::Public(version) } = config::read().unwrap();

    install::by_version(&version);

    let path_var = instantiate_path(&env::var_os("PATH").unwrap(), &version);

    let (exe, args) = command_and_args().unwrap();

    // FIXME: at least in unix, use exec instead
    let status = Command::new(&exe)
        .args(args)
        .env("PATH", path_var)
        .status()
        .unwrap();

    println!("process exited with {}", status);
    // FIXME: exit with the same status code
}