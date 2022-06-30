use std::path::Path;

#[cfg(feature = "bindgen")]
use std::path::PathBuf;

const LIB_NAME: &str = "msdfgen";

fn main() {
    use std::env;

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filename() -> String {
        format!(
            "{}-{}-{}.rs",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        )
    }

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filepath(filename: &str) -> impl AsRef<Path> {
        Path::new("src").join("bindings").join(filename)
    }

    #[cfg(not(feature = "bindgen"))]
    {
        let bindings_file = bindings_filename();

        if bindings_filepath(&bindings_file).as_ref().is_file() {
            println!("cargo:rustc-env=MSDFGEN_BINDINGS={}", bindings_file);
        } else {
            panic!("No prebuilt bindings. Try use `bindgen` feature.",);
        }
    }

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");
    let out_dir = Path::new(&out_dir);

    let src_dir = Path::new("lib");

    #[cfg(feature = "bindgen")]
    {
        //let inc_dirs = try_find_library_inc_dirs().unwrap_or_else(|| vec![src_dir.join("include")]);
        let inc_dirs = vec![src_dir];

        let bindings = out_dir.join("bindings.rs");

        generate_bindings(inc_dirs, &bindings);

        #[cfg(feature = "update-bindings")]
        {
            let out_path = bindings_filepath(&bindings_filename());
            update_bindings(&bindings, &out_path);
        }
    }

    let lib_dir = out_dir;
    build_library(src_dir, lib_dir);
}

#[cfg(feature = "bindgen")]
fn generate_bindings<P: AsRef<Path>>(
    inc_dirs: impl IntoIterator<Item = P>,
    out_file: impl AsRef<Path>,
) {
    let target = std::env::var("TARGET").expect("TARGET is set by cargo.");

    let ctarget = into_c_target(&target);

    let sysroot = detect_sysroot(&target).expect("Detect sysroot.");

    let stdlib = detect_stdlib(&target);

    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .clang_args(stdlib.map_or_else(|| vec![], |stdlib| vec![format!("--stdlib={}", stdlib)]))
        .clang_arg("-target")
        .clang_arg(ctarget)
        .clang_args(sysroot.map_or_else(
            || vec![],
            |sysroot| vec!["-isysroot".into(), sysroot.display().to_string()],
        ))
        .clang_args(
            inc_dirs
                .into_iter()
                .map(|dir| format!("-I{}", dir.as_ref().display())),
        )
        .clang_arg("-DMSDFGEN_USE_CPP11")
        .header("msdfgen.h")
        .header(
            Path::new("src")
                .join("msdfgen_extras.h")
                .display()
                .to_string(),
        )
        .opaque_type("std::.*")
        .allowlist_var("MSDFGEN_.*")
        .allowlist_type("msdfgen::.*")
        .allowlist_function("msdfgen::.*")
        .blocklist_function("msdfgen::(read|write)ShapeDescription")
        .blocklist_type("FILE")
        .blocklist_type("_IO_.*")
        .generate()
        .expect("Generated bindings.");

    bindings.write_to_file(out_file).expect("Written bindings.");
}

#[cfg(feature = "update-bindings")]
fn update_bindings(bind_file: impl AsRef<Path>, dest_file: impl AsRef<Path>) {
    use std::{env, fs, io::Write};

    let dest_file = dest_file.as_ref();

    fs::create_dir_all(&dest_file.parent().unwrap()).unwrap();
    fs::copy(&bind_file, &dest_file).unwrap();

    if let Ok(github_env) = env::var("GITHUB_ENV") {
        let mut env_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(github_env)
            .unwrap();
        writeln!(
            env_file,
            "MSDFGEN_SYS_BINDINGS_FILE={}",
            dest_file.display()
        )
        .unwrap();
    }
}

fn build_library(src_dir: &Path, lib_dir: &Path) {
    let extra_src_dir = Path::new("src");
    let extra_srcs = ["msdfgen_extras.cpp"]
        .into_iter()
        .map(|name| extra_src_dir.join(name));

    let core_src_dir = src_dir.join("core");

    let core_srcs = [
        "contour-combiners.cpp",
        "Contour.cpp",
        "edge-coloring.cpp",
        "EdgeHolder.cpp",
        "edge-segments.cpp",
        "edge-selectors.cpp",
        "equation-solver.cpp",
        "msdf-error-correction.cpp",
        "MSDFErrorCorrection.cpp",
        "msdfgen.cpp",
        "Projection.cpp",
        "rasterization.cpp",
        "render-sdf.cpp",
        "Scanline.cpp",
        "sdf-error-estimation.cpp",
        "Shape.cpp",
        "shape-description.cpp",
        "SignedDistance.cpp",
        "Vector2.cpp",
    ]
    .into_iter()
    .map(|name| core_src_dir.join(name));

    let profile = std::env::var("PROFILE").expect("PROFILE is set by cargo.");

    let mut build = cc::Build::new();

    build
        .cpp(true)
        .opt_level(if profile == "release" { 3 } else { 1 })
        .debug(profile != "release")
        .define(
            if profile == "release" {
                "NDEBUG"
            } else {
                "DEBUG"
            },
            None,
        )
        .define("MSDFGEN_USE_CPP11", None)
        .flag("-std=c++11")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .warnings(false)
        .extra_warnings(false)
        .include(&src_dir)
        .include(&core_src_dir)
        .files(core_srcs)
        .files(extra_srcs)
        .out_dir(lib_dir);

    /*
    if cfg!(feature = "libcxx") {
        build.cpp_set_stdlib("c++");
        build.cargo_metadata(false);
    }
    */

    #[cfg(feature = "shared")]
    build.shared_flag(true);

    #[cfg(feature = "static")]
    build.static_flag(true);

    build.compile(LIB_NAME);
}

#[cfg(feature = "bindgen")]
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

#[cfg(feature = "bindgen")]
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
            .output()
            .map_err(|err| err.to_string())?
            .stdout;

        let str_path =
            std::str::from_utf8(&raw_path).map_err(|_| "Invalid apple SDK path string")?;

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

#[cfg(feature = "bindgen")]
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
