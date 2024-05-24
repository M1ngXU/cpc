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

    for file in fs::read_dir("src").expect("Failed to read `src`.") {
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
            r#"{INDENTPP}"{filename}": {{
{INDENTP}"scope": "rust",
{INDENTP}"prefix": "{filename}",
{INDENTP}"body": ["#,
        )
        .expect("Failed to write start of template json.");
        if filename != "template" {
            writeln!(output, "{INDENT}\"$0\",").expect("Failed to write cursor position.");
        }
        for line in fs::read_to_string(file.path())
            .expect("Failed to read file.")
            .lines()
        {
            let line = format!("{line:?}")
                .replace('$', "\\\\$")
                .replace("// START HERE", "$0");
            writeln!(output, "{INDENT}\"{}\",", &line[1..line.len() - 1])
                .expect("Failed to write to output.");
        }
        writeln!(output, "{INDENTP}]")
            .expect("Failed to write closing bracket for `body` to output.");
        writeln!(output, "{INDENTPP}}},")
            .expect("Failed to write closing bracket for `snippet` line to output.");
    }
    writeln!(output, "{END}").expect("Failed to write `END`");

    output.flush().unwrap();
}
