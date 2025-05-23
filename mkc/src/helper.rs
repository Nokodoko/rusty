
fn chdr(name String){
let new_dir : std::path::PathBuf = current_dir.join(name);
let new_dir_string : String = new_dir.to_str().unwrap().to_owned();
std::env::set_current_dir(new_dir_string).expect("Failed to change current directory to new directory");
}
