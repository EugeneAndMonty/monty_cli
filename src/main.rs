mod global;
mod utils;
mod host_port_setup;
mod background_tasks;
mod console;

use global::{
    DESCRIPTION, BACKGROUNDTASKS, BACKGROUNDTASKS_PROMPT, SETUP, 
    SETUP_PROMPT, HOME, HELPROMPT, HOME_PROMPT, HUNT, HUNT_PROMPT, 
    MONTY, CONSOLE, CONSOLEPROMPT,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = MONTY, about = DESCRIPTION)]
enum Command {
    #[structopt(name = HUNT, about = HUNT_PROMPT)]
    Hunt,
    #[structopt(name = HOME, about = HOME_PROMPT)]
    Home,
    #[structopt(about = HELPROMPT)]
    Help,
    #[structopt(name = SETUP, about = SETUP_PROMPT)]
    Setup {
        #[structopt(short, long)]
        host: String,
        #[structopt(short, long)]
        port: u32,
        #[structopt(short, long)]
        max_connections: Option<u16>, //implement max_connections ?????
    },
    #[structopt(name = BACKGROUNDTASKS, about = BACKGROUNDTASKS_PROMPT)]
    BackGroundTasks {
        #[structopt(short, long)]
        checkexpiration: u64,
    },
    #[structopt(name = CONSOLE, about = CONSOLEPROMPT)]
    Console,
}

fn main() {
    let command = Command::from_args();
    match command {
        Command::Hunt => utils::start_subprocess(),
        Command::Home => utils::stop_subprocess(),
        Command::Help => eprintln!("{}", HELPROMPT),
        Command::Setup { host, port, max_connections} => {
            let _result: Result<(), std::io::Error> = host_port_setup::save_setup(&host, &port, max_connections);
        }
        Command::BackGroundTasks { checkexpiration } => {
            let _result = background_tasks::save_background_tasks_setup(checkexpiration);
        }
        Command::Console => {
            let _result: () = console::start_console();
        }
    }
}