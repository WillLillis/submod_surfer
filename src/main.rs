mod gitmodule;

use crate::gitmodule::{Gitmodule, Submodule};
use std::{env::current_dir, path::PathBuf};

use clap::Parser;

use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use gitmodule::get_gitmodules;

#[derive(Parser, Debug, Clone)]
struct SurferArgs {
    #[arg(long, short, help = "Path to the .gitmodules file of interest")]
    pub module_path: Option<PathBuf>,
    #[arg(
        long,
        short,
        help = "Default: \"%n\", Name: %n or %N, Path: %p, Url: %u, Branch: %b"
    )]
    pub fmt: Option<String>,
}

#[derive(Debug, Clone)]
struct SurferOpts {
    pub module_path: PathBuf,
    pub fmt: String,
}

impl TryFrom<SurferArgs> for SurferOpts {
    type Error = anyhow::Error;
    fn try_from(value: SurferArgs) -> std::result::Result<Self, Self::Error> {
        let module_path = if let Some(user_path) = value.module_path {
            let path = user_path.canonicalize().map_err(|e| {
                anyhow!(
                    "Failed to canonicalize input path \"{}\" -- {e}",
                    user_path.display()
                )
            })?;
            if !path.exists() {
                return Err(anyhow!(
                    "Path \"{}\" does not point to an existing file",
                    path.display()
                ));
            }
            if !path.is_file() || path.is_symlink() {
                return Err(anyhow!(
                    "Path \"{}\" does not point to a valid .gitmodules file",
                    path.display()
                ));
            }

            path
        } else {
            find_gitmodules()?
        };
        let fmt = value.fmt.unwrap_or_else(|| "%n".to_string());

        Ok(Self { module_path, fmt })
    }
}

/// Entry point, parse and validate user arguments
fn main() -> Result<()> {
    let opts: SurferOpts = SurferArgs::parse().try_into()?;
    run(&opts)?;

    Ok(())
}

/// Find the `.gitmodules` file, prompt the user to make a selection, and output
/// the full path to be picked up by our wrapper script
fn run(opts: &SurferOpts) -> Result<()> {
    let gitmodules = get_gitmodules(&opts.module_path)?;
    if gitmodules.is_empty() {
        return Err(anyhow!(
            "No valid submodules detected in file \"{}\"",
            opts.module_path.display()
        ));
    }

    let module = prompt_modules(opts, &gitmodules);
    let mut full_path = opts.module_path.parent().unwrap().to_path_buf();
    full_path.push(&module.path);

    println!("{}", full_path.display());
    Ok(())
}

/// Prompts the user to select an entry out of `modules`, displaying each according
/// to `opts.fmt`
fn prompt_modules<'a>(opts: &SurferOpts, modules: &'a Gitmodule) -> &'a Submodule {
    let choices: Vec<String> = modules.iter().map(|m| m.display_fmt(&opts.fmt)).collect();
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select submodule")
        .default(0)
        .items(&choices[..])
        .interact()
        .unwrap();

    &modules[selection]
}

/// Searchs upward from the current directory, trying to find a `.gitmodules` file
fn find_gitmodules() -> Result<PathBuf> {
    let mut cur_path =
        current_dir().map_err(|e| anyhow!("Failed to detect the current directory -- {e}"))?;

    let mut mod_path =
        PathBuf::with_capacity(cur_path.to_str().unwrap_or_default().len() + ".gitmodules".len());
    loop {
        mod_path.clone_from(&cur_path.clone());
        mod_path.push(".gitmodules");
        if mod_path.exists() && !mod_path.is_symlink() {
            return Ok(mod_path);
        }
        cur_path = if let Some(path) = cur_path.parent() {
            path.to_path_buf()
        } else {
            return Err(anyhow!("Failed to find a parent .gitmodules file"));
        }
    }
}
