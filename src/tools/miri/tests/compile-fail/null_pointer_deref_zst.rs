// Some optimizations remove ZST accesses, thus masking this UB.
// compile-flags: -Zmir-opt-level=0

#[allow(deref_nullptr)]
fn main() {
    let x: () = unsafe { *std::ptr::null() }; //~ ERROR memory access failed: 0x0 is not a valid pointer
    panic!("this should never print: {:?}", x);
}
