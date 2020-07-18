mod error;
mod gpio;

pub use error::*;
pub use gpio::*;

pub type GpioResult<T> = Result<T, GpioError>;
