extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=snap7");

    let bindings = bindgen::Builder::default()
        .header("snap7.h")
        .whitelist_function("Cli_Create")
        .whitelist_function("Cli_Destroy")
        .whitelist_function("Cli_SetConnectionType")
        .whitelist_function("Cli_SetConnectionParams")
        .whitelist_function("Cli_GetParam")
        .whitelist_function("Cli_SetParam")
        .whitelist_function("Cli_ConnectTo")
        .whitelist_function("Cli_GetConnected")
        .whitelist_function("Cli_Disconnect")
        //
        .whitelist_function("Cli_ReadArea")
        .whitelist_function("Cli_WriteArea")
        .whitelist_function("Cli_DBRead")
        .whitelist_function("Cli_DBWrite")
        .whitelist_function("Cli_ABRead")
        .whitelist_function("Cli_ABWrite")
        .whitelist_function("Cli_EBRead")
        .whitelist_function("Cli_EBWrite")
        .whitelist_function("Cli_MBRead")
        .whitelist_function("Cli_MBWrite")
        .whitelist_function("Cli_TMRead")
        .whitelist_function("Cli_TMWrite")
        .whitelist_function("Cli_CTRead")
        .whitelist_function("Cli_CTWrite")
        .whitelist_function("Cli_ReadMultiVars")
        .whitelist_function("Cli_WriteMultiVars")
        //
        .whitelist_function("Cli_ListBlocks")
        .whitelist_function("Cli_ListBlocksOfType")
        .whitelist_function("Cli_GetAgBlockInfo")
        .whitelist_function("Cli_GetPgBlockInfo")
        //
        .whitelist_function("Cli_GetPduLength")
        .whitelist_function("Cli_GetCpuInfo")
        .whitelist_function("Cli_GetCpInfo")
        .whitelist_function("Cli_GetPlcStatus")
        //
        .whitelist_function("Cli_ErrorText")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
