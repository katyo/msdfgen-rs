#[cfg(feature = "generate-bindings")]
mod source {
    pub const URL: &str = "https://github.com/Chlumsky/{package}/archive/{version}.tar.gz";
    //pub const VERSION: &str = "v1.6";
    pub const VERSION: &str = "master";
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

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_arg("-xc++")
            .clang_arg("-std=c++11")
            .clang_arg(format!("-I{}", inc_dir.display()))
            .clang_arg("-DMSDFGEN_USE_CPP11")
            .header(inc_dir.join("msdfgen.h").display().to_string())
            .header(Path::new("src").join("msdfgen_extras.h").display().to_string())
            .opaque_type("std::vector")
            .whitelist_var("MSDFGEN_.*")
            .whitelist_type("msdfgen::.*")
            .whitelist_function("msdfgen::.*")
            .generate()
            .expect("Generated bindings.");

        bindings
            .write_to_file(out_file)
            .expect("Written bindings.");
    }
}
