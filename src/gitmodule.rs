use std::path::PathBuf;

use anyhow::{anyhow, Result};

const PATH_ASSIGN: &str = "path = ";
const URL_ASSIGN: &str = "url = ";
const BRANCH_ASSIGN: &str = "branch = ";

pub type Gitmodule = Vec<Submodule>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Submodule {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub branch: Option<String>,
}

impl Submodule {
    fn try_new(
        name: &Option<String>,
        path: &Option<PathBuf>,
        url: &Option<String>,
        branch: &Option<String>,
    ) -> Result<Self> {
        if let (Some(ref name), Some(ref path), Some(ref url), branch) =
            (&name, &path, &url, branch)
        {
            Ok(Self {
                name: name.to_string(),
                path: path.clone(),
                url: url.to_string(),
                branch: branch.clone(),
            })
        } else {
            Err(anyhow!(
                "Missing fields{}{}{}",
                if name.is_none() { " -- name" } else { "" },
                if path.is_none() { " -- path" } else { "" },
                if url.is_none() { " -- url" } else { "" }
            ))
        }
    }

    pub fn display_fmt(&self, fmt: &str) -> String {
        fmt.replace(
            "%n",
            self.name.rsplit(std::path::MAIN_SEPARATOR).next().unwrap(),
        )
        .replace("%N", &self.name)
        .replace("%p", self.path.to_str().unwrap_or_default())
        .replace("%u", &self.url)
        .replace("%b", self.branch.as_ref().unwrap_or(&String::new()))
    }
}

pub fn get_gitmodules(path: &PathBuf) -> Result<Gitmodule> {
    let contents = std::fs::read_to_string(path)?;
    Ok(parse_gitmodules(&contents))
}

fn parse_gitmodules(contents: &str) -> Gitmodule {
    let clear_state = |name: &mut Option<String>,
                       path: &mut Option<PathBuf>,
                       url: &mut Option<String>,
                       branch: &mut Option<String>| {
        *name = None;
        *path = None;
        *url = None;
        *branch = None;
    };

    let mut gitmodules = Gitmodule::new();

    let mut is_invalid = false; // use to skip past entries with invalid info
    let mut current_name: Option<String> = None;
    let mut current_path: Option<PathBuf> = None;
    let mut current_url: Option<String> = None;
    let mut current_branch: Option<String> = None;

    for line in contents.lines().map(str::trim).filter(|l| !l.is_empty()) {
        // start of a new entry
        if line.starts_with('[') {
            // add the previous one if we gathered all the necessary info
            if !is_invalid {
                if let Ok(module) =
                    Submodule::try_new(&current_name, &current_path, &current_url, &current_branch)
                {
                    gitmodules.push(module);
                }
            }
            is_invalid = false;
            clear_state(
                &mut current_name,
                &mut current_path,
                &mut current_url,
                &mut current_branch,
            );
            if !line.starts_with("[submodule") {
                is_invalid = true;
                continue;
            }
            let Some(first_quote) = line.find('\"') else {
                is_invalid = true;
                continue;
            };
            let Some(second_quote) = line[first_quote + 1..].find('\"') else {
                is_invalid = true;
                continue;
            };

            match &line[first_quote + 1..=(first_quote + second_quote)] {
                "" => is_invalid = true,
                name => current_name = Some(name.to_string()),
            }
        } else if is_invalid {
            continue;
        } else if let Some(path) = line.strip_prefix(PATH_ASSIGN).map(str::trim) {
            current_path = Some(path.into());
        } else if let Some(url) = line.strip_prefix(URL_ASSIGN).map(str::trim) {
            current_url = Some(url.to_string());
        } else if let Some(branch) = line.strip_prefix(BRANCH_ASSIGN).map(str::trim) {
            current_branch = Some(branch.to_string());
        }
    }

    if let Ok(module) =
        Submodule::try_new(&current_name, &current_path, &current_url, &current_branch)
    {
        gitmodules.push(module);
    }

    gitmodules
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::path::PathBuf;

    use crate::gitmodule::{get_gitmodules, Submodule};

    #[test]
    fn it_renders_submodules_correctly() {
        let sample1 = Submodule {
            name: "NameWithoutSlashes".to_string(),
            path: PathBuf::from("/"),
            url: "www.example.com".to_string(),
            branch: None,
        };
        assert_eq!(sample1.display_fmt("%n"), "NameWithoutSlashes");
        assert_eq!(sample1.display_fmt("%N"), "NameWithoutSlashes");
        assert_eq!(sample1.display_fmt("%p"), "/");
        assert_eq!(sample1.display_fmt("%u"), "www.example.com");
        assert_eq!(sample1.display_fmt("%b"), "");
        assert_eq!(sample1.display_fmt("%n (%b)"), "NameWithoutSlashes ()");

        let sample2 = Submodule {
            name: "Name/With/Slashes".to_string(),
            path: PathBuf::from("/"),
            url: "../../somethingfunky.git".to_string(),
            branch: Some("my-feature-branch".to_string()),
        };
        assert_eq!(sample2.display_fmt("%n"), "Slashes");
        assert_eq!(sample2.display_fmt("%N"), "Name/With/Slashes");
        assert_eq!(sample2.display_fmt("%p"), "/");
        assert_eq!(sample2.display_fmt("%u"), "../../somethingfunky.git");
        assert_eq!(sample2.display_fmt("%b"), "my-feature-branch");
        assert_eq!(
            sample2.display_fmt("%n (%b)"),
            "Slashes (my-feature-branch)"
        );
    }

    #[test]
    fn it_parses_gitmodules_correctly() -> Result<()> {
        let gitmodules = get_gitmodules(&PathBuf::from("src/samples/rust-gitmodules"))?;
        let expected = vec![
            Submodule {
                name: "src/doc/nomicon".to_string(),
                path: PathBuf::from("src/doc/nomicon"),
                url: "https://github.com/rust-lang/nomicon.git".to_string(),
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

        assert_eq!(gitmodules.len(), expected.len());
        for (parsed, expected) in gitmodules.iter().zip(expected.iter()) {
            assert_eq!(*parsed, *expected);
        }
        Ok(())
    }
}
