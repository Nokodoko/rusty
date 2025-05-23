use std::process::Command;

fn main() {
    //.Users[]|.UserName
    let users: Vec<&str> = vec!["aws", "iam", "list-users"];
    let aws_users = Command::new(users[0])
        .args(&users[1..])
        .output()
        .expect("Failed to run aws iam list-users");

    if aws_users.status.success() {
        let users: serde_json::Value = serde_json::from_slice(&aws_users.stdout).unwrap();
        if let Some(user_name) = users.get("Users").and_then(|d| d.as_array()) {
            for entry in user_name {
                if let Some(users) = entry.get("UserName") {
                    println!("{}", users);
                }
            }
        }
    } else {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&aws_users.stderr));
    }
}
