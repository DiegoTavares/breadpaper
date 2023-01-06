use crate::bpp::Note;
use std::fmt::{Display, Formatter};

pub mod bpp;

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:}: {:}", self.title, self.content).as_str())
    }
}
