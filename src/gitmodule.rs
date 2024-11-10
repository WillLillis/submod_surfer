use std::path::PathBuf;

use anyhow::{anyhow, Result};

const PATH_ASSIGN: &str = "path = ";
const URL_ASSIGN: &str = "url = ";
const BRANCH_ASSIGN: &str = "branch = ";

pub type Gitmodule = Vec<Submodule>;

#[derive(Debug, Clone)]
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
    let clear_state = |name: &mut Option<String>,
                       path: &mut Option<PathBuf>,
                       url: &mut Option<String>,
                       branch: &mut Option<String>| {
        *name = None;
        *path = None;
        *url = None;
        *branch = None;
    };

    let contents = std::fs::read_to_string(path)?;
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

    Ok(gitmodules)
}
