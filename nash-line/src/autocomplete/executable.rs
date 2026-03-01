use super::{Completer, trie::Trie};
use std::{os::unix::fs::PermissionsExt, vec};

pub struct ExecutableCompleter {
    completions: Trie,
}

impl ExecutableCompleter {
    pub fn new() -> Self {
        Self {
            completions: Trie::new(),
        }
    }

    pub fn with_executables_in_path(mut self) -> Self {
        let executables = find_executables();

        for exe in executables {
            self.completions.insert(&exe);
        }

        self
    }
}

#[cfg(debug_assertions)]
impl ExecutableCompleter {
    pub fn trie(&self) -> &Trie {
        &self.completions
    }
}

impl Completer for ExecutableCompleter {
    fn complete(&self, s: &str) -> Vec<String> {
        if let Some(completions) = self.completions.get_completions(s) {
            return completions;
        }

        vec![]
    }
}

fn find_executables() -> Vec<String> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let mut executables = Vec::new();

    for dir in std::env::split_paths(&path_var) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let is_executable = entry
                        .metadata()
                        .map(|m| m.permissions().mode() & 0o111 != 0)
                        .unwrap_or(false);

                    if is_executable {
                        if let Some(name) = path.file_name() {
                            executables.push(name.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }

    executables
}
