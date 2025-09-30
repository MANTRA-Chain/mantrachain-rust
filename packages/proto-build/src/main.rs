//! Build Mantrachain proto files. This build script clones the CosmosSDK and Mantrachain version
//! specified in the COSMOS_SDK_REV and MANTRACHAIN_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{fs, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

const COSMOS_SDK_REPO: &str = "https://github.com/MANTRA-Chain/cosmos-sdk.git";
const CONNECT_REPO: &str = "https://github.com/MANTRA-Chain/connect.git";
const EVM_REPO: &str = "https://github.com/MANTRA-Chain/evm.git";
const MANTRACHAIN_REPO: &str = "https://github.com/MANTRA-Chain/mantrachain.git";

/// The mantrachain commit or tag to be cloned and used to build the proto files
const MANTRACHAIN_REV: &str = "v5.0.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the connect submodule is located
const CONNECT_DIR: &str = "../../dependencies/connect/";
/// Directory where the evm submodule is located
const EVM_DIR: &str = "../../dependencies/evm/";
/// Directory where the mantrachain submodule is located
const MANTRACHAIN_DIR: &str = "../../dependencies/mantrachain/";

/// A temporary directory for repos storing
const TMP_REPOS_DIR: &str = "../../dependencies/";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";
/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../mantrachain-std/src/types/";

pub fn generate() {
    let tmp_repos_dir: PathBuf = TMP_REPOS_DIR.parse().unwrap();
    if tmp_repos_dir.exists() {
        fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
    }

    git::clone_repo(MANTRACHAIN_REPO, MANTRACHAIN_DIR, MANTRACHAIN_REV);

    // Extract versions from go.mod with specific module types
    let cosmos_sdk_rev = git::get_version_from_go_mod(MANTRACHAIN_DIR, git::Module::CosmosSdk)
        .expect("Failed to get Cosmos SDK version");
    let connect_rev = git::get_version_from_go_mod(MANTRACHAIN_DIR, git::Module::Connect)
        .expect("Failed to get Connect version");
    let evm_rev = git::get_version_from_go_mod(MANTRACHAIN_DIR, git::Module::Evm)
        .expect("Failed to get EVM version");

    git::clone_repo(COSMOS_SDK_REPO, COSMOS_SDK_DIR, &cosmos_sdk_rev);
    git::clone_repo(CONNECT_REPO, CONNECT_DIR, &connect_rev);
    git::clone_repo(EVM_REPO, EVM_DIR, &evm_rev);

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: cosmos_sdk_rev,
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec![
            "autocli".to_string(),
            "benchmark".to_string(),
            "counter".to_string(),
            "reflection".to_string(),
        ],
    };

    let connect_project = CosmosProject {
        name: "connect".to_string(),
        version: connect_rev.to_string(),
        project_dir: CONNECT_DIR.to_string(),
        exclude_mods: vec![],
    };

    let evm_project = CosmosProject {
        name: "evm".to_string(),
        version: evm_rev.to_string(),
        project_dir: EVM_DIR.to_string(),
        exclude_mods: vec![],
    };

    let mantrachain_project = CosmosProject {
        name: "mantrachain".to_string(),
        version: MANTRACHAIN_REV.to_string(),
        project_dir: MANTRACHAIN_DIR.to_string(),
        exclude_mods: vec![],
    };

    let mantrachain_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        mantrachain_project,
        vec![cosmos_project, connect_project, evm_project],
    );

    mantrachain_code_generator.generate();

    fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
