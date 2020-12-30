#![feature(fn_traits)]
pub mod event;
pub mod listener;
pub mod marshal;
pub mod node;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
