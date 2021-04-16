use {
    bindgen::{Builder, CargoCallbacks},
    std::{env::var, path::PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=nakama/include/nakama.h");

    let bindings = Builder::default()
        .header("nakama/include/nakama.h")
        .blacklist_type("size_t")
        .blacklist_type("wchar_t")
        .blacklist_item("max_align_t")
        .blacklist_item("__bool_true_false_are_defined")
        .blacklist_item("false_")
        .blacklist_item("true_")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
