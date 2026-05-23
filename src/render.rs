use owo_colors::OwoColorize;

use crate::scan::Summary;

pub fn render(summary: &Summary, color: bool) -> String {
    let title = if color {
        summary.root_name.bold().cyan().to_string()
    } else {
        summary.root_name.clone()
    };

    let mut lines = vec![
        format!("╭─ {title}"),
        format!("│ LOC       {}", summary.total_code),
        format!("│ Files     {}", summary.total_files),
        format!("│ Languages {}", summary.languages.len()),
    ];

    if !summary.languages.is_empty() {
        lines.push("│".to_string());
        for language in &summary.languages {
            let percent = if summary.total_code == 0 {
                0.0
            } else {
                (language.code as f64 / summary.total_code as f64) * 100.0
            };
            let name = if color {
                language.name.green().to_string()
            } else {
                language.name.clone()
            };
            lines.push(format!(
                "│ {:<12} {:>7} {:>5.1}%",
                name, language.code, percent
            ));
        }
    }

    lines.push("╰".to_string());
    lines.join("\n") + "\n"
}
