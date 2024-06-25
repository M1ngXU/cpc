use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    process::exit,
    sync::Arc,
};

use serde::Deserialize;
use serde_json::{from_str, to_writer_pretty};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpSocket,
    select,
    sync::{mpsc, Mutex},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpSocket::new_v4()?;
    socket.bind(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4244)))?;
    let listener = socket.listen(1024)?;
    let mut counter = 0;
    let (done_tx, mut done_rx) = mpsc::channel::<()>(1);
    let problems = Arc::new(Mutex::new(Vec::<Info>::new()));
    println!("Listening ...");
    loop {
        select! {
            r = listener.accept() => {
                let (mut socket, _) = r?;
                counter += 1;
                let done = done_tx.clone();
                let problems = problems.clone();
                tokio::spawn(async move {
                    println!("Reading {counter} ...");
                    let mut s = String::new();
                    socket
                        .read_to_string(&mut s)
                        .await
                        .expect("Failed to read data.");
                    let body = s
                        .split_once("\r\n\r\n")
                        .expect("No body found.")
                        .1
                        .to_string();
                    let json = tokio::task::spawn_blocking(move || {
                        from_str::<Info>(&body).expect("Failed to parse json.")
                    })
                    .await
                    .expect("Failed to join json thread.");
                    let mut p = problems.lock().await;
                    let d = p.len() + 1 == json.batch.size;
                    if p.is_empty() {
                        println!("Found {} problem(s)!", json.batch.size);
                    } else {
                        if p[0].batch.id != json.batch.id {
                            println!("Not matching batch id, try again!");
                            exit(1);
                        }
                    }
                    p.push(json);
                    std::mem::drop(p);
                    std::mem::drop(problems);
                    if d {
                        done.send(()).await.expect("Failed to send done.");
                    }
                    println!("Done receiving ...");
                });
            }
            r = done_rx.recv() => {
                let () = r.expect("Failed to receive done.");
                break;
            }
        }
    }
    let problems = Arc::into_inner(problems)
        .expect("Some reference to problems left.")
        .into_inner();
    let set_id = problems[0]
        .url
        .split_once("/contest/")
        .unwrap_or_else(|| problems[0].url.split_once("/gym/").unwrap())
        .1
        .split_once('/')
        .unwrap()
        .0
        .to_string();
    println!("Group: {}, set id: `{set_id}`", problems[0].group);
    let dir = PathBuf::from(format!("bin/{set_id}"));
    fs::create_dir_all(&dir).await?;
    let mut toml = OpenOptions::new()
        .append(true)
        .read(true)
        .create(false)
        .open("bin/Cargo.toml")
        .await?;
    let mut toml_str = String::new();
    toml.read_to_string(&mut toml_str).await?;
    let mut problem_info = HashMap::new();
    for problem in &problems {
        let id = problem
            .name
            .split_once('.')
            .map(|x| x.0.to_string())
            .unwrap_or_else(|| problem.name.clone())
            .to_lowercase()
            .replace(['.', ' ', '/'], "_");
        println!("Creating problem with id `{id}` ...");
        let toml_id = format!("{set_id}_{id}");
        assert_eq!(problem_info.insert(id.clone(), toml_id.clone()), None);
        if dir.join(&id).exists() {
            println!("Skipping, already exists ...");
        } else {
            let d = dir.join(&id);
            fs::create_dir(&d).await?;
            let samples = d.join("samples");
            fs::create_dir(&samples).await?;
            for (i, t) in problem.tests.iter().enumerate() {
                fs::write(samples.join(format!("{i:02}.in")), &t.input).await?;
                fs::write(samples.join(format!("{i:02}.out")), &t.output).await?;
            }
            let sol = d.join("sol.rs");
            fs::write(&sol, "fn main() {}").await?;
            println!("Path: {sol:?}");
            if toml_str.contains(&toml_id) {
                println!("Already found in `Cargo.toml`, not adding there ...");
            } else {
                toml.write_all(
                    format!(
                        "[[bin]]
name = \"{toml_id}\"
path = \"{}\"

",
                        sol.to_string_lossy()
                            .replace('\\', "/")
                            .trim_start_matches("bin/")
                    )
                    .as_bytes(),
                )
                .await?;
            }
            println!("Done!");
        }
    }
    println!("Writing `bin/problems.json` ...");
    to_writer_pretty(std::fs::File::create("bin/problems.json")?, &problem_info)
        .expect("Failed to write `bin/problems.json`");
    println!("Finished all!");
    Ok(())
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
struct Info {
    name: String,
    group: String,
    url: String,
    interactive: bool,
    memory_limit: usize,
    time_limit: usize,
    tests: Vec<Test>,
    test_type: TestType,
    batch: Batch,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Test {
    input: String,
    output: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum TestType {
    Single,
    MultiNumber,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Batch {
    id: String,
    size: usize,
}
