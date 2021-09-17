// use std::fmt;

// #[derive(strum_macros::Display)]
pub enum Brac {
    Nan,
    Par,
    Brk,
    Brc,
    Ang,
}

// impl fmt::Display for Brac {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//         // fmt::Debug::fmt(self, f)
//         write!(f, "{:?}", self)
//     }
// }