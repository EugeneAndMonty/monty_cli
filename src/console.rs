use std::io::{self, Read, Write};
use indexmap::IndexMap;
mod console_commands;
use crate::utils;
use std::net::TcpStream;

pub(crate) fn create_generic_indexmap_for_raw_request() -> IndexMap<String, Vec<String>> 
{
    let mut indexmap: IndexMap<String, Vec<String>> = IndexMap::new();
    indexmap.insert("raw".to_string(), Vec::new());
    indexmap.insert("superowner_credentials".to_string(), Vec::new());
    indexmap
}

pub(crate) fn start_console() {

    utils::start_subprocess();

    println!("Enter your command or type 'exit' to quit:");

    loop {

        print!("> ");

        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input: std::str::SplitWhitespace = input.trim().split_whitespace();
        let strings_vec: Vec<&str> = input.collect();

        let mut indexmap: IndexMap<String, Vec<String>> = create_generic_indexmap_for_raw_request();

        match strings_vec[0] {

            "exit" => break,

            "restart" => {
                utils::stop_subprocess();
                utils::start_subprocess();
            },

            "create_superowner" => {
                indexmap["raw"] = strings_vec.iter().map(|x| x.to_string()).collect();
                let _ = run_stream(indexmap);
            },

            "create_owner" => {
                let mut superowner_username_inp: String = String::new();
                let mut superowner_pass_inp: String = String::new();

                print!("Superowner: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut superowner_username_inp).unwrap();
                let superowner_username: &str = superowner_username_inp.trim();

                print!("Password: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut superowner_pass_inp).unwrap();
                let superowner_password: &str = superowner_pass_inp.trim();
                indexmap["raw"] = strings_vec.iter().map(|x| x.to_string()).collect();
                indexmap["superowner_credentials"] = Vec::from([superowner_username.to_string(), superowner_password.to_string()]);

                let _ = run_stream(indexmap);
            },
            "remove_owner" => {
                let _ = run_stream(indexmap);
                // console_commands::ownership::remove_owner(strings_vec)
            },
            "grant_to" => {
                let _ = run_stream(indexmap);
                // console_commands::ownership::grant_to(strings_vec)
            },
            "revoke_from" => {
                let _ = run_stream(indexmap);
                // console_commands::ownership::revoke_from(strings_vec)
            },
            "create_peers_network" => {
                let _ = run_stream(indexmap);
            },
            "whitelist_peers" => {
                let _ = run_stream(indexmap);
            },
            _ => println!("Command is not recognized"),

        }
    }
}

fn run_stream(message: IndexMap<String, Vec<String>>) -> io::Result<()> {

    let server_address: &str = "127.0.0.1:21210"; //open file, read file, get server address

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_address);

           let message: String = format!("{:?}\n", message);
            let _ = stream.write_all(message.as_bytes());

            let mut buffer: [u8; 512] = [0; 512];
            let bytes_read: Result<usize, io::Error> = stream.read(&mut buffer[..]);

            match bytes_read {
                Ok(size) if size > 0 => {
                    let response: std::borrow::Cow<str> = String::from_utf8_lossy(&buffer[..size]);
                    println!("{}", response);
                },
                _ => {
                    println!("No response received");
                },
                
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to the server: {}", e);
        },
    }

    Ok(())

}

// INSERT KEY=example VAL=example TIMER=500 PERSISTENT=false