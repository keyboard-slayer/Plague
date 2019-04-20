use std::process::Command;

fn get_username() -> String {
    let output_utf8 = Command::new("whoami")
                      .output()
                      .expect("Failed to execute");

    let output = String::from_utf8_lossy(&output_utf8.stdout).into_owned();
    output.replace("\n", "")
}

fn get_targets(link: &str) -> String {
    let output_utf8 = Command::new("curl")
                      .arg(link)
                      .output()
                      .expect("Failed to execute");
    let output = String::from_utf8_lossy(&output_utf8.stdout).into_owned();
    output
}
fn main() {
    let username = get_username();
    let targets = get_targets("http://localhost/user.txt");
    println!("Username: {}", username);
    println!("Targets: \n{}", targets);
}
