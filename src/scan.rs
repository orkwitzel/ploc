use std::path::Path;
use tokei::{Config, Languages};

const NOISE_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "node_modules",
    "target",
    "dist",
    "build",
    ".next",
    ".nuxt",
    "vendor",
    ".cache",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageStat {
    pub name: String,
    pub code: usize,
    pub files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub root_name: String,
    pub total_code: usize,
    pub total_files: usize,
    pub languages: Vec<LanguageStat>,
}

pub fn scan_current_dir(include_noise: bool) -> Result<Summary, String> {
    let root =
        std::env::current_dir().map_err(|err| format!("cannot read current directory: {err}"))?;
    scan_path(&root, include_noise)
}

pub fn scan_path(root: &Path, include_noise: bool) -> Result<Summary, String> {
    if !root.is_dir() {
        return Err(format!("{} is not a directory", root.display()));
    }

    let config = Config::default();
    let ignored_directories = if include_noise { &[][..] } else { NOISE_DIRS };

    let mut languages = Languages::new();
    languages.get_statistics(&[root], ignored_directories, &config);

    let mut stats = languages
        .into_iter()
        .filter_map(|(language, reports)| {
            let code = reports.code;
            let files = reports.reports.len();
            (code > 0 || files > 0).then(|| LanguageStat {
                name: language.name().to_string(),
                code,
                files,
            })
        })
        .collect::<Vec<_>>();

    stats.sort_by(|left, right| {
        right
            .code
            .cmp(&left.code)
            .then_with(|| left.name.cmp(&right.name))
    });

    let total_code = stats.iter().map(|stat| stat.code).sum();
    let total_files = stats.iter().map(|stat| stat.files).sum();
    let root_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".")
        .to_string();

    Ok(Summary {
        root_name,
        total_code,
        total_files,
        languages: stats,
    })
}
