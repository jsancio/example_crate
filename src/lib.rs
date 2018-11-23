pub fn a() {}

#[cfg(test)]
mod tests {
    use crate::a;

    #[test]
    fn it_works() {
        a();
    }
}
