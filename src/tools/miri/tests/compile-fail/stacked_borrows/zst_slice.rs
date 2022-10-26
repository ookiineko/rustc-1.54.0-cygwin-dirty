// compile-flags: -Zmiri-track-raw-pointers
// error-pattern: does not have an appropriate item in the borrow stack

fn main() {
    unsafe {
        let a = [1, 2, 3];
        let s = &a[0..0];
        assert_eq!(s.len(), 0);
        assert_eq!(*s.get_unchecked(1), 2);
    }
}
