use crate::config::initial::initial_config;
use doc::gen::start_process;

mod doc;
mod config;

fn main() {
    initial_config();
    start_process();
}
