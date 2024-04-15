use rustbase_scram::ScramClient;
use std::io;

fn send_and_receive(message: &str) -> String {
    // write message to stdout
    println!("{}", message);
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}

fn main() {

    let mut user = "user".to_string();
    let mut password = "123456".to_string();
    if std::env::args().len() > 1 {
        user = std::env::args().nth(1).unwrap();
        password = std::env::args().nth(2).unwrap();
        if "sha256" != std::env::args().nth(3).unwrap() {
            panic!("Only sha256 is supported.")
        }
    }

    let scram = ScramClient::new(&user, &password, None);
    // Get the client message and reassign the SCRAM state.
    let (scram, client_first) = scram.client_first();
    // println!("Client First: {}", client_first);

    let server_first = send_and_receive(&client_first);

    // println!("Server First: {}", server_first);

    let scram = scram.handle_server_first(&server_first).unwrap();

    // Get the client final message and reassign the SCRAM state.
    let (scram, client_final) = scram.client_final();
    // println!("Client Final: {}", client_final);

    let server_final = send_and_receive(&client_final);

    match scram.handle_server_final(&server_final)
        {
           Ok(()) => println!("AUTH OK"),
           Err(_err) => println!("AUTH FAILED")
        }
}
