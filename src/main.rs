mod cli;
mod xpbf;

use crate::cli::cli_process;
use rust_i18n::{i18n, set_locale};
use sys_locale::get_locale;

i18n!("locales");

fn main() {
    set_locale(&get_locale().unwrap_or_else(|| "en_US".to_string()));
    cli_process();
}
