use lazy_static::lazy_static;
use records::Record;
use std::sync::Mutex;

mod config;
mod date;
mod db;
mod records;

lazy_static! {
    static ref CONFIG: Mutex<config::Config> = Mutex::new(config::Config::initialize());
    static ref FAIL_COUNT: Mutex<u8> = Mutex::new(0);
}

fn logs(comment: &str) {
    println!(
        "{}\tLOG: {}",
        chrono::Local::now().format("%d/%m/%Y %H:%M:%S"),
        comment
    );
}

fn error(comment: &str) {
    eprintln!(
        "{}\tERROR: {}",
        chrono::Local::now().format("%d/%m/%Y %H:%M:%S"),
        comment
    );
}

async fn run_test() {
    let db = db::Database::new();
    loop {
        let record = match Record::new() {
            Ok(record) => record,
            Err(records::RecordError::Fatal) => {
                error("Fatal error in Record::new(). Exiting.");
                return;
            }
            Err(records::RecordError::NonFatal(timeout)) => {
                error(&format!(
                    "Non-fatal error in Record::new(). Retrying in {} seconds.",
                    timeout
                ));
                tokio::time::sleep(tokio::time::Duration::from_secs(timeout.into())).await;
                continue;
            }
            Err(records::RecordError::TooBig) => {
                error("RecordError::TooBig in Record::new(). Tring again.");
                continue;
            }
        };

        db.insert_record(record);

        let interval = {
            let config = CONFIG.lock().unwrap();
            config.interval
        };

        tokio::time::sleep(tokio::time::Duration::from_secs((60 * interval).into())).await;
    }
}

#[tokio::main]
async fn main() {
    run_test().await;
    println!("Hello, world!");
}
