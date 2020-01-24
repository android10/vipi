use console::Term;
use std::io::Result;
use std::process::Command;
use std::process::Output;

fn main() {
    setup_ui();

    let auth_code = read_two_factor_auth_code();
    process_two_factor_auth_code(auth_code);
}

fn setup_ui() {
    println!("Two Factor Authentication Code: ");
}

fn read_two_factor_auth_code() -> Result<String> {
    let console = Term::stdout();
    let read_value_result: Result<String> = console.read_line();
    
    return read_value_result;
}

fn process_two_factor_auth_code(result: Result<String>) {
    match result {
        Ok(auth_code) => connect_to_vpn(auth_code.as_str()),
        Err(e)  => println!("Error: {}", e),
    }
}

fn connect_to_vpn(two_factor_auth_code: &str) {
    let output = execute(two_factor_auth_code);
    println!("status: {}", output.status);
}

fn execute(two_factor_auth_code: &str) -> Output {
    //TODO: concatenate password to the file 'twofactorvpn.conf'
    //TODO: where to save data in a safe way
    //TODO: run command with sudo?
    let output = Command::new("sudo openvpn")
                     .arg("--config")
                     .arg("vpn.ovpn")
                     .arg("--auth-user-pass")
                     .arg("twofactorvpn.conf")
                     .arg("--auth-nocache")
                     .output()
                     .expect("failed to execute process");

    assert!(output.status.success());

    return output;
}