use serde_json::json;
use std::{error::Error, process::Output};
use tokio::{self, process::Command};
use warp::{self, Filter};

async fn list_service() -> Result<Output, Box<dyn Error>> {
    let service_cmd: Vec<&str> = vec!["/home/n0ko/forest/scripts/apm/dd_apm_list.py", "list"];
    let cmd_runner = Command::new(&service_cmd[0])
        .arg(&service_cmd[1])
        .output()
        .await?;
    Ok(cmd_runner)
}

#[tokio::main]

async fn main() {
    #[derive(Debug)]
    struct ServerError;
    impl warp::reject::Reject for ServerError {}

    let get_route = warp::path::end().and_then(|| async {
        match list_service().await {
            Ok(output) => {
                let services = String::from_utf8_lossy(&output.stdout);
                Ok(warp::reply::json(&json!({ "services": services })))
            }
            Err(_) => Err(warp::reject::custom(ServerError)),
        }
    });

    let routes = get_route;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
