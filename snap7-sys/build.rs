extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=snap7");

    let bindings = bindgen::Builder::default()
        .header("snap7.h")
        .allowlist_function("Cli_Create")
        .allowlist_function("Cli_Destroy")
        .allowlist_function("Cli_SetConnectionType")
        .allowlist_function("Cli_SetConnectionParams")
        .allowlist_function("Cli_GetParam")
        .allowlist_function("Cli_SetParam")
        .allowlist_function("Cli_ConnectTo")
        .allowlist_function("Cli_GetConnected")
        .allowlist_function("Cli_Disconnect")
        //
        .allowlist_function("Cli_ReadArea")
        .allowlist_function("Cli_WriteArea")
        .allowlist_function("Cli_DBRead")
        .allowlist_function("Cli_DBWrite")
        .allowlist_function("Cli_ABRead")
        .allowlist_function("Cli_ABWrite")
        .allowlist_function("Cli_EBRead")
        .allowlist_function("Cli_EBWrite")
        .allowlist_function("Cli_MBRead")
        .allowlist_function("Cli_MBWrite")
        .allowlist_function("Cli_TMRead")
        .allowlist_function("Cli_TMWrite")
        .allowlist_function("Cli_CTRead")
        .allowlist_function("Cli_CTWrite")
        .allowlist_function("Cli_ReadMultiVars")
        .allowlist_function("Cli_WriteMultiVars")
        //
        .allowlist_function("Cli_ListBlocks")
        .allowlist_function("Cli_ListBlocksOfType")
        .allowlist_function("Cli_GetAgBlockInfo")
        .allowlist_function("Cli_GetPgBlockInfo")
        //
        .allowlist_function("Cli_GetPduLength")
        .allowlist_function("Cli_GetCpuInfo")
        .allowlist_function("Cli_GetCpInfo")
        .allowlist_function("Cli_GetPlcStatus")
        //
        .allowlist_function("Cli_ErrorText")
        // Support dynamic loading
        .dynamic_library_name("LibSnap7")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
