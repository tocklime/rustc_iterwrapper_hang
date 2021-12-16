fn main() {
    let mut iter = vec![1, 2].into_iter();
    func(&mut iter);
}
fn func(n: &mut impl Iterator<Item = usize>) {
    let i = n.next().unwrap();
    if i == 1 {
        let mut wrapped = IterWrap { inner: n };
        func(&mut wrapped);
    } else {
        println!("{}", i);
    }
}
pub struct IterWrap<'a, V, T: Iterator<Item = V>> {
    inner: &'a mut T,
}
impl<'a, V, T: Iterator<Item = V>> Iterator for IterWrap<'a, V, T> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
