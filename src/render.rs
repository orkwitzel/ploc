use owo_colors::{OwoColorize, XtermColors};

use crate::scan::Summary;

const BAR_WIDTH: usize = 24;
const PALETTE: &[u8] = &[208, 39, 112, 178, 141, 35, 197, 69, 45, 220];

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
        lines.push(format!("│ {}", language_bar(summary, color)));
        lines.push("│".to_string());
        let name_width = summary
            .languages
            .iter()
            .map(|language| language.name.chars().count())
            .max()
            .unwrap_or(0);
        let code_width = summary
            .languages
            .iter()
            .map(|language| language.code.to_string().len())
            .max()
            .unwrap_or(1);

        for (index, language) in summary.languages.iter().enumerate() {
            let name = format!("{:<name_width$}", language.name);
            let name = colorize(&name, index, color);
            lines.push(format!("│ {} {:>code_width$}", name, language.code));
        }
    }

    lines.push("╰".to_string());
    lines.join("\n") + "\n"
}

fn language_bar(summary: &Summary, color: bool) -> String {
    let segments = language_segments(summary);
    let mut bar = String::new();

    for (index, width) in segments {
        let segment = "█".repeat(width);
        bar.push_str(&colorize(&segment, index, color));
    }

    bar
}

fn language_segments(summary: &Summary) -> Vec<(usize, usize)> {
    if summary.total_code == 0 {
        return Vec::new();
    }

    let mut raw_segments = summary
        .languages
        .iter()
        .enumerate()
        .filter(|(_, language)| language.code > 0)
        .map(|(index, language)| {
            let exact = language.code as f64 * BAR_WIDTH as f64 / summary.total_code as f64;
            let whole = exact.floor() as usize;
            (index, whole, exact - whole as f64)
        })
        .collect::<Vec<_>>();

    let used = raw_segments
        .iter()
        .map(|(_, whole, _)| *whole)
        .sum::<usize>();
    let mut remaining = BAR_WIDTH.saturating_sub(used);

    raw_segments.sort_by(|left, right| {
        right
            .2
            .partial_cmp(&left.2)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.0.cmp(&right.0))
    });

    for (_, width, _) in &mut raw_segments {
        if remaining == 0 {
            break;
        }
        *width += 1;
        remaining -= 1;
    }

    raw_segments.sort_by_key(|(index, _, _)| *index);
    raw_segments
        .into_iter()
        .filter_map(|(index, width, _)| (width > 0).then_some((index, width)))
        .collect()
}

fn colorize(text: &str, index: usize, color: bool) -> String {
    if color {
        text.color(XtermColors::from(PALETTE[index % PALETTE.len()]))
            .to_string()
    } else {
        text.to_string()
    }
}
