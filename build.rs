use npm_rs::NpmEnv;
use std::env;

fn main() {
    // https://kit.svelte.dev/docs/project-structure#other-files-svelte-kit
    println!("cargo::rerun-if-changed=ui/src");
    println!("cargo::rerun-if-changed=ui/static");
    println!("cargo::rerun-if-changed=ui/package.json");
    println!("cargo::rerun-if-changed=ui/svelte.config.js");
    println!("cargo::rerun-if-changed=ui/tsconfig.json");
    println!("cargo::rerun-if-changed=ui/vite.config.ts");

    let exit_status = NpmEnv::default()
        .set_path(format!(
            "{}/ui",
            env::var("CARGO_MANIFEST_DIR").expect("missing env variable")
        ))
        .init_env()
        .install(None)
        .run("check")
        .run("build")
        .exec()
        .expect("npm command failed");
    assert!(exit_status.success());
}
