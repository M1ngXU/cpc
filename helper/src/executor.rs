use std::{collections::HashMap, env::args, fs::File, path::PathBuf, process::Stdio};

use itertools::Itertools;
use serde_json::from_reader;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
    process::Command,
    time::Instant,
};

/// From: https://github.com/TheAlgorithms/Rust/blob/master/src/dynamic_programming/longest_common_subsequence.rs
///
/// Longest common subsequence via Dynamic Programming
/// longest_common_subsequence(a, b) returns the longest common subsequence
/// between the strings a and b.
pub fn longest_common_subsequence(a: &str, b: &str) -> Vec<(u8, char)> {
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();
    let (na, nb) = (a.len(), b.len());

    // solutions[i][j] is the length of the longest common subsequence
    // between a[0..i-1] and b[0..j-1]
    let mut solutions = vec![vec![0; nb + 1]; na + 1];

    for (i, ci) in a.iter().enumerate() {
        for (j, cj) in b.iter().enumerate() {
            // if ci == cj, there is a new common character;
            // otherwise, take the best of the two solutions
            // at (i-1,j) and (i,j-1)
            solutions[i + 1][j + 1] = if ci == cj {
                solutions[i][j] + 1
            } else {
                solutions[i][j + 1].max(solutions[i + 1][j])
            }
        }
    }
    // reconstitute the solution string from the lengths
    let mut result: Vec<(u8, char)> = Vec::new();
    let (mut i, mut j) = (na, nb);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push((0, a[i - 1]));
            i -= 1;
            j -= 1;
        } else if solutions[i - 1][j] > solutions[i][j - 1] {
            i -= 1;
            result.push((1, a[i]));
        } else {
            j -= 1;
            result.push((2, b[j]));
        }
    }
    while j > 0 {
        j -= 1;
        result.push((2, b[j]));
    }
    while i > 0 {
        i -= 1;
        result.push((1, a[i]));
    }

    result.reverse();
    result.into_iter().collect_vec()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let problems: HashMap<String, String> =
        from_reader(File::open("bin/problems.json")?).expect("Failed to read `problems.json`.");
    println!("Found `{}` problems.", problems.len());

    let problem_id = args().nth(1).expect("Should be `id [-i (interactive)]`.");
    let interactive = args().any(|x| x == "-i");
    let actual = problems
        .get(&problem_id)
        .expect("Didn't find problem with that id.")
        .clone();
    println!("Executing `{problem_id}` (`{actual}`) (interactive: `{interactive}`) ...");

    let backtrace = args().any(|a| a == "-b");

    let cmd = || {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "-p", "bin", "--bin"]).arg(&actual);
        if backtrace {
            cmd.env("RUST_BACKTRACE", "1");
        }
        cmd
    };
    if interactive {
        let mut child = cmd()
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        println!("Running, enter stdin:");
        println!("==========");
        std::mem::forget(child.stdin.take());
        let output = child.wait_with_output().await?;
        println!("====STDOUT====");
        println!(
            "{}",
            String::from_utf8(output.stdout).expect("Output not utf-8.")
        );
        println!("==END=STDOUT==");
        if output.status.success() {
            println!("Successfully executed.");
        } else {
            panic!("Unsuccessful execution!");
        }
    } else {
        let dir = PathBuf::from(format!("bin/{}/samples", actual.replace('_', "/")));
        let mut entries = fs::read_dir(&dir).await?;
        while let Some(file) = entries.next_entry().await? {
            if file.file_type().await.is_ok_and(|ft| ft.is_file()) {
                let path = file.path();
                if path.extension().is_some_and(|e| e == "in") {
                    println!("Found in: `{path:?}`");
                    let out = path.with_extension("out");
                    if !out.exists() {
                        panic!("Expected output file does not exist (`{out:?}`).");
                    }
                    let input = fs::read(path).await?;
                    let expected =
                        String::from_utf8(fs::read(out).await?).expect("Non utf-8 sample output.");
                    let mut cmd = cmd();
                    cmd.stdin(Stdio::piped());
                    cmd.stdout(Stdio::piped());
                    cmd.stderr(Stdio::inherit());
                    let mut child = cmd.spawn()?;
                    let start = Instant::now();
                    child.stdin.take().unwrap().write_all(&input).await?;
                    let output = child.wait_with_output().await?;
                    let execution_time = start.elapsed();
                    if output.status.success() {
                        println!("Successfully executed in {execution_time:?}.");
                        let actual = String::from_utf8(output.stdout).expect("Non utf-8 output.");
                        if actual
                            .split_ascii_whitespace()
                            .eq(expected.split_ascii_whitespace())
                        {
                            println!("Ok!");
                        } else {
                            println!("Output does not match:");
                            println!("========");
                            for (i, line) in
                                expected.lines().zip_longest(actual.lines()).enumerate()
                            {
                                let i = i + 1;
                                let (e, a) = line.left_and_right();
                                if e == a {
                                    println!("\x1b[1mOK-{i:02}>\x1b[m {}", e.unwrap());
                                } else {
                                    match (e, a) {
                                        (Some(e), Some(a)) => {
                                            print!("\x1b[1m!!-{i:02}>\x1b[m ");
                                            for (t, c) in longest_common_subsequence(e, a) {
                                                match t {
                                                    0 => print!("{c}"),
                                                    1 => print!("\x1b[30;42m{c}\x1b[m"),
                                                    2 => print!("\x1b[9;30;41m{c}\x1b[m"),
                                                    _ => unreachable!(),
                                                }
                                            }
                                            println!();
                                        }
                                        (Some(e), None) => {
                                            println!("\x1b[1m++-{i:02}>\x1b[m \x1b[30;42m{e}\x1b[m")
                                        }
                                        (None, Some(a)) => {
                                            println!(
                                                "\x1b[1m---{i:02}>\x1b[m \x1b[9;30;41m{a}\x1b[m\n"
                                            )
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                            }
                            println!("========");
                        }
                    } else {
                        println!("====STDOUT====");
                        println!(
                            "{}",
                            String::from_utf8(output.stdout).expect("Output not utf-8.")
                        );
                        println!("==END=STDOUT==");
                        panic!("Unsuccessful execution!");
                    }
                }
            } else {
                println!("Found non-file: `{file:?}`");
            }
        }
    }

    Ok(())
}
