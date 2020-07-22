mod base;
mod client;
mod config;
mod reader;
mod test_client;
mod writer;

pub use base::*;
pub use client::*;
pub use config::*;
pub use reader::*;
pub use test_client::*;
pub use writer::*;

pub fn chomp(src: &str) -> &str {
    let mut out = src;

    if out.len() > 0 && &out[out.len() - 1..] == "\n" {
        out = &out[..out.len() - 1]
    }

    return out;
}

#[cfg(test)]
mod tests {
    use crate::chomp;

    #[test]
    fn test_chomp() {
        assert_eq!("a", chomp("a\n"));
        assert_eq!("a", chomp("a"));
        assert_eq!("a\n", chomp("a\n\n"));
        assert_eq!("", chomp(""));
    }
}
