use {
    bindgen::{Builder, CargoCallbacks},
    std::{env::var, path::PathBuf},
};

fn main() {
    let bindings = Builder::default()
        .header("nakama/include/nakama.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
