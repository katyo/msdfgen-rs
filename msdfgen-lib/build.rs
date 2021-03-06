mod source {
    pub const URL: &str = "https://github.com/katyo/{package}-rs/releases/download/{package}-core-{version}/{package}-core-{version}.tar.gz";
    pub const VERSION: &str = "1.6-gitb9d6f0b";
}

fn main() {
    #[cfg(not(feature = "rustdoc"))]
    {
        use std::{
            env,
            path::Path,
        };

        let src = utils::Source::new(
            "msdfgen",
            env::var("MSDFGEN_VERSION")
                .unwrap_or(source::VERSION.into()),
            env::var("MSDFGEN_URL")
                .unwrap_or(source::URL.into()),
        );

        let out_dir = env::var("OUT_DIR")
            .expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source")
            .join(&src.version);

        utils::fetch_source(&src, &src_dir);

        utils::compile_library(&src_dir, &out_dir);
    }
}

mod utils {
    use std::path::Path;

    pub struct Source {
        pub package: String,
        pub version: String,
        pub url: String,
    }

    impl Source {
        pub fn new(package: impl Into<String>, version: impl Into<String>, url: impl Into<String>) -> Self {
            Self { package: package.into(), version: version.into(), url: url.into() }
        }

        pub fn url(&self) -> String {
            self.url.replace("{package}", &self.package).replace("{version}", &self.version)
        }
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) {
        use fetch_unroll::Fetch;

        if !out_dir.is_dir() {
            let src_url = src.url();

            eprintln!("Fetch msdfgen from {} to {}",
                      src_url, out_dir.display());

            Fetch::from(src_url).unroll().strip_components(1).to(out_dir)
                .expect("Msdfgen sources should be fetched.");
        }
    }

    pub fn lib_file<S: AsRef<str>>(name: S, shared: bool) -> String {
        #[cfg(target_os = "windows")]
        {
            format!("{}.{}", name.as_ref(), if shared { "dll" } else { "lib" })
        }

        #[cfg(not(target_os = "windows"))]
        {
            format!("lib{}.{}", name.as_ref(), if shared { "so" } else { "a" })
        }
    }

    pub fn lib_link<S: AsRef<str>>(name: S, shared: Option<bool>) {
        println!("cargo:rustc-link-lib={}{}",
                 if let Some(shared) = shared {
                     if shared { "dylib=" } else { "static=" }
                 } else { "" },
                 name.as_ref());
    }

    pub fn compile_library(src_dir: &Path, out_dir: &Path) {
        let lib_dir = out_dir;
        let lib_name = "msdfgen";
        let lib_file = lib_dir.join(lib_file(&lib_name, cfg!(feature = "shared")));
        let extra_src_dir = Path::new("src");
        let extra_srcs = &["msdfgen_extras.cpp"].iter()
            .map(|extra_src| extra_src_dir.join(extra_src))
            .collect::<Vec<_>>();

        if !lib_file.is_file() {
            /*std::fs::create_dir_all(out_dir).unwrap();

            #[cfg(feature = "cmake")]
            {
                pub fn bool_flag(flag: bool) -> &'static str {
                    if flag { "ON" } else { "OFF" }
                }

                cmake::Config::new(src_dir)
                .define("CMAKE_C_COMPILER_WORKS", bool_flag(true))
                .define("CMAKE_CXX_COMPILER_WORKS", bool_flag(true))
                .always_configure(true)
                .very_verbose(true)
                .out_dir(out_dir)
                .build();
            }*/

            //#[cfg(feature = "cc")]
            {
                let cpp_ext = Some(std::ffi::OsStr::new("cpp"));

                let include_dir = src_dir.join("include");
                let core_src_dir = src_dir.join("core");

                let core_srcs = std::fs::read_dir(&core_src_dir)
                    .expect("Source directory")
                    .filter_map(Result::ok)
                    .filter_map(|entry| {
                        let path = entry.path();
                        if path.is_file() && path.extension() == cpp_ext {
                            path.into()
                        } else {
                            None
                        }
                    });

                let profile = std::env::var("PROFILE").expect("PROFILE is set by cargo.");

                cc::Build::new()
                    .cpp(true)
                    .opt_level(if profile == "release" { 3 } else { 1 })
                    .debug(profile != "release")
                    .flag(if profile == "release" { "-DNDEBUG" } else { "-DDEBUG" })
                    .define("MSDFGEN_USE_CPP11", None)
                    .flag("-std=c++11")
                    .flag("-ffunction-sections")
                    .flag("-fdata-sections")
                    .warnings(false)
                    .extra_warnings(false)
                    .include(&src_dir)
                    .include(&include_dir)
                    .include(&core_src_dir)
                    .files(core_srcs)
                    .files(extra_srcs)
                    .shared_flag(cfg!(feature = "shared"))
                    .static_flag(!cfg!(feature = "shared"))
                    .cargo_metadata(false)
                    .compile(&lib_name);
            }
        }

        for extra_src in extra_srcs {
            println!("cargo:rerun-if-changed={}", extra_src.display());
        }

        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        lib_link(lib_name, None);

        lib_link(if cfg!(feature = "libcxx") { "c++" } else { "stdc++" },
                 if cfg!(feature = "stdcxx-static") { Some(false) } else { None });
    }
}
