#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::gitmodule::{get_gitmodules, Submodule};

    use super::*;

    #[test]
    fn it_parses_gitmodules_correctly() {
        let gitmodules = get_gitmodules("samples/rust-gitmodules")?;
        let expected = vec![
            Submodule {
                name: "src/doc/nomicon".to_string(),
                path: PathBuf::from("src/doc/nomicon"),
                url: "".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/tools/cargo".to_string(),
                path: PathBuf::from("src/tools/cargo"),
                url: "https://github.com/rust-lang/cargo.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/doc/reference".to_string(),
                path: PathBuf::from("src/doc/reference"),
                url: "https://github.com/rust-lang/reference.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/doc/book".to_string(),
                path: PathBuf::from("src/doc/book"),
                url: "https://github.com/rust-lang/book.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/doc/rust-by-example".to_string(),
                path: PathBuf::from("src/doc/rust-by-example"),
                url: "https://github.com/rust-lang/rust-by-example.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "library/stdarch".to_string(),
                path: PathBuf::from("library/stdarch"),
                url: "https://github.com/rust-lang/stdarch.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/doc/rustc-dev-guide".to_string(),
                path: PathBuf::from("src/doc/rustc-dev-guide"),
                url: "https://github.com/rust-lang/rustc-dev-guide.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/doc/edition-guide".to_string(),
                path: PathBuf::from("src/doc/edition-guide"),
                url: "https://github.com/rust-lang/edition-guide.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/llvm-project".to_string(),
                path: PathBuf::from("src/llvm-project"),
                url: "https://github.com/rust-lang/llvm-project.git".to_string(),
                branch: Some("rustc/19.1-2024-09-17".to_string()),
            },
            Submodule {
                name: "src/doc/embedded-book".to_string(),
                path: PathBuf::from("src/doc/embedded-book"),
                url: "https://github.com/rust-embedded/book.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "library/backtrace".to_string(),
                path: PathBuf::from("library/backtrace"),
                url: "https://github.com/rust-lang/backtrace-rs.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/tools/rustc-perf".to_string(),
                path: PathBuf::from("src/tools/rustc-perf"),
                url: "https://github.com/rust-lang/rustc-perf.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/tools/enzyme".to_string(),
                path: PathBuf::from("src/tools/enzyme"),
                url: "https://github.com/EnzymeAD/Enzyme.git".to_string(),
                branch: None,
            },
            Submodule {
                name: "src/gcc".to_string(),
                path: PathBuf::from("src/gcc"),
                url: "https://github.com/rust-lang/gcc.git".to_string(),
                branch: None,
            },
        ];
        assert_eq!(gitmodules, expected);
    }
}
