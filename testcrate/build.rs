use pluto_build as pluto;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    pluto::Build::new()
        .opt_ilp_enabled()
        .opt_load_hook("contmod_on_load")
        .compile();
}
