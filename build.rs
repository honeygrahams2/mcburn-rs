use dirs::home_dir;
use std::{
    env::{self}, 
    path::PathBuf, 
    process::Command,
};

use include_idl::compress_idl;

fn main() {   
    // Run shank to generate the IDL
    let idl_dir = home_dir().unwrap().join("projects/mcburn/idl");
    let _output = Command::new("shank")
        .arg("idl")
        .arg("-o")
        .arg(idl_dir)
        .arg("-p")
        .arg("BtFLxrtCs4BR43jpHvZ9o4t3xk9zkKXx4JTuDhFTeD8W")
        .output()
        .expect("Failed to run shank");

    // Get the IDL path
    let idl_path = home_dir().unwrap().join("projects/mcburn/idl/mcburn.json");

    // Concat output path of compressed IDL
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("idl.json.zip");
    
    compress_idl(&idl_path, &dest_path);
}