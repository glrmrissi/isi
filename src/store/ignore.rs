use std::fs;
use std::io;

use super::repo::find_root;

pub struct IsiIgnore {
    patterns: Vec<String>,
}

impl IsiIgnore {
    pub fn load() -> Self {
        let patterns = Self::read_patterns().unwrap_or_default();
        IsiIgnore { patterns } 
    }

    fn read_patterns() -> io::Result<Vec<String>> {
        let root = find_root()?;
        let ignore_path = root.join(".isiignore");

        if !ignore_path.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(ignore_path)?;
        let patterns = content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();

        Ok(patterns)
    }

    pub fn should_ignore(&self, rel_path: &str, is_dir: bool) -> bool {
        self.patterns
            .iter()
            .any(|p| pattern_matches(p, rel_path, is_dir))
    }
}

fn pattern_matches(pattern: &str, rel_path: &str, is_dir: bool) -> bool {
    if let Some(dir_pattern) = pattern.strip_suffix('/') {
        if !is_dir {
            return false;
        }
        return component_match(dir_pattern, rel_path)
            || glob_match(dir_pattern, rel_path);
    }

    if pattern.contains('/') {
        return glob_match(pattern, rel_path);
    }

    component_match(pattern, rel_path) || glob_match(pattern, rel_path)
}

fn component_match(pattern: &str, path: &str) -> bool {
    path.split('/').any(|part| glob_match(pattern, part))
}

fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_inner(&p, &t)
}

fn glob_inner(p: &[char], t: &[char]) -> bool {
    match (p.first(), t.first()) {
        (None, None) => true,
        (Some('*'), _) => {
            for i in 0..=t.len() {
                if t[..i].contains(&'/') {
                    break;
                }
                if glob_inner(&p[1..], &t[i..]) {
                    return true;
                }
            }
            false
        }
        (None, _) | (_, None) => false,
        (Some(pc), Some(tc)) => pc == tc && glob_inner(&p[1..], &t[1..]),
    }
}
