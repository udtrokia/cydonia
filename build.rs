//! This `build.rs` will generate substrate-parachain-template to embed
use etc::{Etc, FileSystem, Meta, Tree};
use std::{path::PathBuf, process::Command};

const SUBSTRATE_PARACHIN_TEMPLATE: &str =
    "https://github.com/substrate-developer-hub/substrate-parachain-template.git";
const SUBSTRATE_PARACHAIN_TEMPLATE_DIR: &str = "substrate_parachain_template";

/// Check and return home dir of cydonia
fn check_home() -> PathBuf {
    let sys_home = Etc::from(dirs::home_dir().expect("Could not find home_dir"));
    sys_home
        .mkdir(".cydonia")
        .expect("Could not create ~/.cydonia");

    sys_home
        .open(".cydonia")
        .expect("Coud not open ~/.cydonia")
        .real_path()
        .expect("~/.cydonia does not exist")
}

/// Batch the parachain template
///
/// if exits:
///   update
/// else:
///   download
fn download_template(root: &PathBuf) -> Tree {
    let mut git = Command::new("git");
    if root.exists() {
        git.args(&[
            "-C",
            &root
                .join(SUBSTRATE_PARACHAIN_TEMPLATE_DIR)
                .to_string_lossy(),
            "pull",
        ])
    } else {
        git.args(&[
            "clone",
            SUBSTRATE_PARACHIN_TEMPLATE,
            &root
                .join(SUBSTRATE_PARACHAIN_TEMPLATE_DIR)
                .to_string_lossy(),
        ])
    }
    .status()
    .expect(&format!("Could not clone {}", SUBSTRATE_PARACHIN_TEMPLATE));

    Tree::batch(&Etc::from(root.join(SUBSTRATE_PARACHAIN_TEMPLATE_DIR)))
        .expect("Batch template failed")
}

// main func
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let root = check_home();
    let _tree = download_template(&root);
}
