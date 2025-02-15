#![doc = include_str!("../README.md")]

use std::str::FromStr;

const FILES: &[&str] = &[
    "lapi",
    "lcode",
    "lctype",
    "ldebug",
    "ldo",
    "ldump",
    "lfunc",
    "lgc",
    "llex",
    "lmem",
    "lobject",
    "lopcodes",
    "lparser",
    "lstate",
    "lstring",
    "ltable",
    "ltm",
    "lundump",
    "lvm",
    "lzio",
    "lauxlib",
    "lbaselib",
    "lcorolib",
    "ldblib",
    "liolib",
    "lmathlib",
    "loadlib",
    "loslib",
    "lstrlib",
    "lcryptolib",
    "ltablib",
    "lutf8lib",
    "lassertlib",
    "lvector3lib",
    "lbase32",
    "lbase64",
    "ljson",
    "lurllib",
    "linit",
    "lstarlib",
    "lcatlib",
    "lhttplib",
    "lschedulerlib",
    "lsocketlib",
    "lbigint",
    "lxml",
    "lregex",
    "lffi",
    "lcanvas",
];

mod options;

pub struct Build(cc::Build);

impl Build {
    pub fn new() -> Self {
        let mut build = cc::Build::new();

        build.cpp(true).std("c++17").opt_level(3);
        build.warnings(false).pic(true);

        build.define("SOUP_USE_INTRIN", None);
        build.define("_CRT_SECURE_NO_WARNINGS", None);
        build.define("NDEBUG", None);
        build.define("_CONSOLE", None);
        build.flag_if_supported("/Zc:__cplusplus");
        build.flag_if_supported("/wd4646");

        if cfg!(debug_assertions) {
            build.opt_level(0);
            build.define("LUA_USE_APICHECK", None);
        }

        // update the package version in Cargo.toml to match the Pluto version!
        let (_, version) = env!("CARGO_PKG_VERSION").split_once('-').unwrap();
        let src_dir = std::path::PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
        let src_dir = src_dir.join(format!("Pluto-{version}/src"));

        build_soup_dependencies(&build, &src_dir);

        let target = std::env::var("TARGET").unwrap();
        match target {
            _ if target.contains("linux") => {
                build.define("LUA_USE_LINUX", None);
                println!("cargo:rustc-link-lib=dl");
            }
            _ if target.contains("apple-darwin") => {
                build
                    .define("LUA_USE_MACOSX", None)
                    .define("LUA_USE_READLINE", None);
                println!("cargo:rustc-link-lib=resolv");
                println!("cargo:rustc-link-lib=readline");
            }
            _ if target.ends_with("bsd") => {
                build
                    .define("LUA_USE_LINUX", None)
                    .define("LUA_USE_READLINE", None)
                    .include("/usr/include/edit");
                println!("cargo:rustc-link-lib=edit");
            }
            _ if target.contains("windows") => {
                build.define("LUA_USE_WINDOWS", None);
            }
            _ => panic!("don't know how to build Pluto for {}", target),
        };

        build.include(&src_dir);
        for file in FILES {
            build.file(src_dir.join(file).with_extension("cpp"));
        }

        Self(build)
    }

    pub fn compile(&mut self) {
        self.0.compile("plutostatic");
        let out_dir = std::env::var("OUT_DIR").unwrap();
        println!("cargo:rustc-link-search=native={out_dir}");
        println!("cargo:rustc-link-lib=static=plutostatic");
    }
}

fn build_soup_dependencies(b: &cc::Build, src_dir: &std::path::Path) {
    let soup_intrin_src = src_dir.join("vendor/Soup/Intrin");
    b.clone()
        .include(&soup_intrin_src)
        .add_files_by_ext(&soup_intrin_src, "cpp")
        .flags(&[
            "-maes", "-mavx", "-mavx2", "-mpclmul", "-mrdrnd", "-mrdseed", "-msha", "-msse4.1",
        ])
        .compile("soupintrin");

    let soup_src = src_dir.join("vendor/Soup/soup");
    b.clone()
        .include(&soup_src)
        .add_files_by_ext(&soup_src, "cpp")
        .compile("soup");

    println!("cargo:rustc-link-lib=static=soup");
    println!("cargo:rustc-link-lib=static=soupintrin");
}

trait BuildExt {
    fn add_files_by_ext(&mut self, dir: &std::path::Path, ext: &str) -> &mut Self;
    fn flags(&mut self, flags: &[&str]) -> &mut Self;
}

impl BuildExt for cc::Build {
    fn add_files_by_ext(&mut self, dir: &std::path::Path, ext: &str) -> &mut Self {
        for entry in std::fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension() == Some(ext.as_ref()))
        {
            self.file(entry.path());
        }
        self
    }

    fn flags(&mut self, flags: &[&str]) -> &mut Self {
        for flag in flags {
            self.flag_if_supported(flag);
        }
        self
    }
}
