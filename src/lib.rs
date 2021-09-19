#[macro_use]
extern crate lazy_static;

pub use capitalize::capitalize;
pub use charset::{has_ansi, has_astral, has_han, REG_ANSI, REG_ASTRAL, REG_HAN, strip_ansi, strip_astral};
pub use concat::{Concat, concat_lines, concat_lines_edged, contingent_lines, join};
pub use lange::lange;
pub use str_value::str_value;
pub use sub::sub;

pub mod enums;
pub mod chars;

mod capitalize;
mod str_value;
mod sub;
mod concat;
mod bracket;
mod charset;
mod lange;