//use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir_name = &args[1];
    std::fs::create_dir(dir_name).expect("Unable to make directory");
    std::env::set_current_dir(dir_name).expect("Unable to switch to new directory")


    //Command::new("cd")
    //    //stringify dir_name
    //    .arg()
    //    .output()
    //    .expect("Unable to change directory");

    //match env::set_current_dir(dir_name){
    //    Ok(_) => println!("{:?}", env::current_dir()),
    //   Err(e) => eprintln!("Unable to move to new directory {}", e),
    //}

//    if let Err(e) = env::set_current_dir(dir_name){
//        println!("Unable to change directory, {}", e);
//    } else {
//        println!("Current directory {:?}", env::current_dir());
//    }

//    let pwd_command = Command::new("pwd")
//        .output()
//        .expect("Unable to print present working directory");
//
//    println!("{}", String::from_utf8_lossy(&pwd_command.stdout));
}
