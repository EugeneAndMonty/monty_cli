pub(crate) const MONTY: &str = "monty";
pub(crate) const HUNT: &str = "hunt";
pub(crate) const HOME: &str = "home";
pub(crate) const SETUP: &str = "setup";
pub(crate) const BACKGROUNDTASKS: &str = "background";
pub(crate) const CONSOLE: &str = "console";

pub(crate) const BACKGROUNDTASKS_PROMPT: &str = "Set expiration check rate";
pub(crate) const HUNT_PROMPT: &str = "Start the engine";
pub(crate) const HOME_PROMPT: &str = "Stop the engine";
pub(crate) const CONSOLEPROMPT: &str = "Call console";

pub(crate) const HELPROMPT: &str = 
"'monty hunt' command to start engine
\n'monty home' to stop engine
\n'monty superowner --username <username> --password <password>' to setup superowner credentials
\n 'monty setup --host <host> --port <port> --max-connections <max-connections>' to setup host, port and max number of connections
\n 'monty background --checkexpiration <checkexpiration>' to setup expiration check rate";

pub(crate) const DESCRIPTION: &str = "Database engine commands";
pub(crate) const SETUP_PROMPT: &str = "Setup host, port and max number of connections";

//UNIX BASED PATHS
pub(crate) const MONTY_DB_PATH: &str = "/Users/user/Desktop/monty_db/target/debug/monty_db";
//pub(crate) const FILE_PATH_SUPEROWNER: &str = "/Users/eugene/Desktop/project/monty_cli/target/debug/credentials.bin"; //credentials.bin
pub(crate) const FILE_PATH_SETUP: &str = "/Users/user/Desktop/monty_cli/target/debug/config.bin"; //config.bin
pub(crate) const FILE_PATH_BACKGROUND_TASKS: &str = "/Users/user/Desktop/monty_cli/target/debug/background.bin"; //background.bin

//WINDOWS BASED PATHS
// pub(crate) const MONTY_DB_PATH: &str = "C:\\Users\\esukharev\\Desktop\\new_folder\\monty_db\\target\\debug\\monty_db.exe";
// pub(crate) const FILE_PATH_SETUP: &str = "C:\\Users\\esukharev\\Desktop\\new_folder\\monty_cli\\target\\debug\\config.bin";
// pub(crate) const FILE_PATH_BACKGROUND_TASKS: &str = "C:\\Users\\esukharev\\Desktop\\new_folder\\monty_cli\\target\\debug\\background.bin";


pub(crate) const PAGE_SIZE: usize = 4096;
