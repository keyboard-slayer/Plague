use std::process::Command;

fn run_command(command : &str, argument : &str) -> String {
    let output_utf8 = Command::new(command)
                      .arg(argument)
                      .output()
                      .expect("Failed to execute");
    String::from_utf8_lossy(&output_utf8.stdout).into_owned()
}


fn main() {
    let username      : String    = run_command("id", "-un").replace("\n", "");
    let targets       : String    = run_command("curl", "http://localhost/user.txt");
    let targets_vec   : Vec<&str> = targets.lines().collect();
    let mut in_target : bool      = false;

    for target in targets_vec {
        if username == target {
            in_target = true;
        }
    }
}
