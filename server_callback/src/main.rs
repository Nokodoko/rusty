use std::io;
use std::process::Command;
use std::process::Stdio;
use tokio;
use warp::{self, Filter};

#[tokio::main]
async fn main() {
    let api_data = warp::path!("api" / "data").and(warp::get().map(|| {
        let result = match kubectx() {
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        };
        warp::reply::json(&result)
    }));

    warp::serve(api_data).run(([127, 0, 0, 1], 3033)).await;
}

fn kubectx() -> io::Result<String> {
    let ctx: &str = "kubectx";
    let ctx_process = Command::new(ctx).stdout(Stdio::piped()).spawn()?;

    if let Some(ctx_stdout) = ctx_process.stdout {
        let dmenu_cmd: Vec<&str> = vec![
            "dmenu",
            "-m",
            "0",
            "-fn",
            "VictorMono:size=20",
            "-nf",
            "green",
            "-nb",
            "black",
            "-nf",
            "yellow",
            "-sb",
            "black",
        ];
        let dmenu_process = Command::new(&dmenu_cmd[0])
            .args(&dmenu_cmd[1..])
            .stdin(Stdio::from(ctx_stdout))
            .stdout(Stdio::piped())
            .spawn()?;
        let output = dmenu_process.wait_with_output()?;
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(output_str)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to list files"))
    }
}
