use std::fs;
use std::io::Write;

const START: &str = r#"{"#;
const END: &str = r#"}"#;
const INDENT_TABS: usize = 3;
const INDENTPP: &str = "\t";
const INDENTP: &str = "\t\t";
const INDENT: &str = "\t\t\t";

fn main() {
    std::env::set_current_dir(std::env::args().nth(1).unwrap());

    let mut output =
        fs::File::create(".vscode/rust.code-snippets").expect("Failed to open output file.");

    writeln!(output, "{START}").expect("Failed to write `START`");

    for (i, file) in fs::read_dir("src")
        .expect("Failed to read `src`.")
        .enumerate()
    {
        let file = file.expect("Failed to read `file`.");
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let filename = file
            .file_name()
            .into_string()
            .unwrap()
            .trim_end_matches(".rs")
            .to_string();
        if filename == "main" || filename == "lib" {
            continue;
        }
        writeln!(
            output,
            r#"{}{INDENTPP}"{filename}": {{
{INDENTP}"scope": "rust",
{INDENTP}"prefix": "{filename}",
{INDENTP}"body": ["#,
            if i == 0 { "" } else { ",\n" }
        )
        .expect("Failed to write start of template json.");
        if filename != "template" {
            writeln!(output, "{INDENT}\"$0\",").expect("Failed to write cursor position.");
        }
        writeln!(
            output,
            "{}",
            fs::read_to_string(file.path())
                .expect("Failed to read file.")
                .lines()
                .map(|line| format!("{line:?}")
                    .replace('$', "\\\\$")
                    .replace("// START HERE", "$0"))
                .map(|line| format!("{INDENT}\"{}\"", &line[1..line.len() - 1]))
                .collect::<Vec<_>>()
                .join(",\n")
        )
        .expect("Failed to write to output.");
        writeln!(output, "{INDENTP}]")
            .expect("Failed to write closing bracket for `body` to output.");
        write!(output, "{INDENTPP}}}")
            .expect("Failed to write closing bracket for `snippet` line to output.");
    }
    writeln!(output, "\n{END}").expect("Failed to write `END`");

    output.flush().unwrap();
}
