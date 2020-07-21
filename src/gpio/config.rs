use tokio::fs::File;
#[derive(Debug)]
pub struct Config {
    pub open: bool,
    pub err_if_already_opened: bool,
    pub close_if_open: bool,
    pub gpio_n: usize,
    pub gpio_n_str: Option<String>,
    pub value_path: Option<String>,
    pub file: Option<File>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            open: true,
            err_if_already_opened: false,
            close_if_open: true,
            gpio_n: 0,
            gpio_n_str: None,
            value_path: None,
            file: None,
        }
    }
}