extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    match pkg_config::probe_library("libevdev") {
        Ok(lib) => {
            for path in &lib.include_paths {
                println!("cargo:include={}", path.display());
            }

            let bindings = bindgen::Builder::default()
                .header("wrapper.h")
                .generate()
                .expect("Unable to generate bindings");

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
            return
        },
        Err(e) => {
            println!("err: {:?}", e);
            panic!()
        } ,
    }

}
