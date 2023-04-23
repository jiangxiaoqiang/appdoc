use std::collections::HashMap;
use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;
use chrono::Local;

pub fn initial_config() {
    initial_file_config();
}

pub fn get_config(key: &str) -> String {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let hash_config = settings.try_into::<HashMap<String, String>>().unwrap();
    let conn = hash_config.get(key).unwrap();
    let std =String::from(conn);
    return std;
}

pub fn initial_log_config(){
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Error)
        .init();
}

pub fn initial_file_config(){
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());
}
