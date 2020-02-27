#[cfg(feature = "generate-bindings")]
mod source {
    pub const URL: &str = "https://github.com/katyo/{package}-rs/releases/download/{package}-core-{version}/{package}-core-{version}.tar.gz";
    pub const VERSION: &str = "1.6-gitb9d6f0b";
}

fn main() {
    #[cfg(feature = "generate-bindings")]
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

        let inc_dir = src_dir;
        let bindings = out_dir.join("bindings.rs");

        utils::generate_bindings(&inc_dir, &bindings);
    }
}

#[cfg(feature = "generate-bindings")]
mod utils {
    use std::path::{Path, PathBuf};

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

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let target = std::env::var("TARGET")
            .expect("TARGET is set by cargo.");

        let ctarget = into_c_target(&target);

        let sysroot = detect_sysroot(&target)
            .expect("Detect sysroot.");

        let stdlib = detect_stdlib(&target);

        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_arg("-xc++")
            .clang_arg("-std=c++11")
            .clang_args(stdlib.map_or_else(|| vec![], |stdlib| vec![format!("--stdlib={}", stdlib)]))
            .clang_arg("-target").clang_arg(ctarget)
            .clang_args(sysroot.map_or_else(|| vec![], |sysroot| vec!["-isysroot".into(), sysroot.display().to_string()]))
            .clang_arg(format!("-I{}", inc_dir.display()))
            .clang_arg("-DMSDFGEN_USE_CPP11")
            .header(inc_dir.join("msdfgen.h").display().to_string())
            .header(Path::new("src").join("msdfgen_extras.h").display().to_string())
            .opaque_type("std::.*")
            .whitelist_var("MSDFGEN_.*")
            .whitelist_type("msdfgen::.*")
            .whitelist_function("msdfgen::.*")
            .blacklist_function("msdfgen::(read|write)ShapeDescription")
            .blacklist_type("FILE")
            .blacklist_type("_IO_.*")
            .generate()
            .expect("Generated bindings.");

        bindings
            .write_to_file(out_file)
            .expect("Written bindings.");
    }

    fn into_c_target(target: &str) -> String {
        use std::env;

        let mut result_target = target.to_string();

        if target.contains("-android") {
            let android_api = env::var("ANDROID_PLATFORM")
                .or(env::var("ANDROID_API"))
                .unwrap_or("16".into());

            result_target.push_str(&android_api);
        }

        result_target
    }

    fn detect_sysroot(target: &str) -> Result<Option<PathBuf>, String> {
        use std::env;

        println!("cargo:rerun-if-env-changed=TARGET_SYSROOT");

        if let Ok(path) = env::var("TARGET_SYSROOT") {
            return Ok(Some(path.into()));
        }

        if target.contains("-apple-") {
            let sdk_name = if target.contains("-darwin") {
                "macosx"
            } else if target.contains("-ios") {
                if target.contains("i386") || target.contains("x86_64") {
                    "iphonesimulator"
                } else {
                    "iphoneos"
                }
            } else {
                return Err(format!("Invalid apple target: {}", target));
            };

            let raw_path = std::process::Command::new("xcrun")
                .args(&["--sdk", sdk_name, "--show-sdk-path"])
                .output().map_err(|err| err.to_string())?
                .stdout;

            let str_path = std::str::from_utf8(&raw_path)
                .map_err(|_| "Invalid apple SDK path string")?;

            Ok(Some(str_path.trim().into()))
        } else if target.contains("-android") {
            let ndk_home = env::var("ANDROID_NDK_HOME")
                .or(env::var("NDK_HOME"))
                .map_err(|_| "Either ANDROID_NDK_HOME or NDK_HOME should be set.".to_string())?;

            let ndk_path = Path::new(&ndk_home);

            //let ndk_bin = ndk_path.join("toolchains/llvm/prebuilt/linux-x86_64/bin");

            Ok(Some(ndk_path.join("sysroot").into()))
        } else {
            Ok(None)
        }
    }

    fn detect_stdlib(target: &str) -> Option<String> {
        use std::env;

        println!("cargo:rerun-if-env-changed=CXX_STDLIB");

        if let Ok(stdlib) = env::var("CXX_STDLIB") {
            return Some(stdlib);
        }

        if target.contains("-ios") {
            return Some("libc++".into());
        }

        None
    }
}
