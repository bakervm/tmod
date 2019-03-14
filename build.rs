use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=stock/src/main.bst");

    Command::new("tame").arg("build").current_dir("stock").spawn().expect("unable to compile stock rom");
}