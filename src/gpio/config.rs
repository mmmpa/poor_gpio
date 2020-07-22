#[derive(Debug, Clone)]
pub struct Config {
    pub open: bool,
    pub err_if_already_opened: bool,
    pub close_if_open_self: bool,
    pub gpio_n: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            open: true,
            err_if_already_opened: false,
            close_if_open_self: true,
            gpio_n: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedConfig {
    pub close: bool,
    pub gpio_n: usize,
    pub gpio_n_str: String,
    pub value_path: String,
}

impl From<Config> for NormalizedConfig {
    fn from(c: Config) -> Self {
        let Config {
            open,
            close_if_open_self: close_if_open,
            gpio_n,
            ..
        } = c;

        Self {
            close: open && close_if_open,
            gpio_n,
            gpio_n_str: "".to_string(),
            value_path: "".to_string(),
        }
    }
}
