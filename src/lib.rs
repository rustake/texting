pub use _capitalize::capitalize;
pub use _str_value::str_value;
pub use _sub::sub;

mod _capitalize;
mod _str_value;
mod _sub;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
