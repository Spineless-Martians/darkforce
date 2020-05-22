use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=app/src");
    println!("cargo:rerun-if-changed=app/gatsby-browser.js");
    println!("cargo:rerun-if-changed=app/gatsby-config.js");
    println!("cargo:rerun-if-changed=app/gatsby-node.js");
    println!("cargo:rerun-if-changed=app/gatsby-ssr.js");
    println!("cargo:rerun-if-changed=app/package.json");
    println!("cargo:rerun-if-changed=app/tsconfig.json");
    println!("cargo:rerun-if-changed=app/yarn.lock");

    if !Command::new("yarn")
        .current_dir("app")
        .spawn()
        .expect("Is yarn installed?")
        .wait()
        .unwrap()
        .success()
    {
        panic!("Failed to install app dependencies")
    }

    if !Command::new("yarn")
        .arg("build")
        .current_dir("app")
        .spawn()
        .expect("Is yarn installed?")
        .wait()
        .unwrap()
        .success()
    {
        panic!("Failed to build gatsby site")
    }
}
