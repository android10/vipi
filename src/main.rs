use console::Term;
use std::io::Result;

fn main() {
    process_two_factor_auth_code(read_two_factor_auth_code());
}

fn read_two_factor_auth_code() -> Result<String> {
    println!("Two Factor Authentication Code: ");

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
    println!("{}", two_factor_auth_code);
}
