#[derive(Debug, Clone)]
pub struct Config {
    pub open: bool,
    pub err_if_already_opened: bool,
    pub close_if_open: bool,
    pub gpio_n: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            open: true,
            err_if_already_opened: false,
            close_if_open: true,
            gpio_n: 0,
        }
    }
}
