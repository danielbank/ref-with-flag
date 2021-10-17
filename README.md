# RefWithFlag

An interesting bit-level hacking example using [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) from the book [Programming Rust](https://www.oreilly.com/library/view/programming-rust/9781491927274/) by Jim Blandy and Jason Orendorff. The code takes advantage of the fact that many types have to be placed at even addresses in memory, so that the least significant bit (LSB) is always zero. If a type satisfies this constraint, we can utilize the LSB to store the value of a flag while still maintaining the information of the original memory address. When we set the RefWithFlag `ptr_and_bit`, we bitwise-and the flag with the LSB. When we get reference, we mask off the flag value. When we read the flag, we just return whether the LSB was 0 or 1.

In the example, `PhantomData` is necessary for Rust to know how to treat lifetimes in code that use `RefWithFlag`. Without it, the code won't compile:

```
// This won't compile
pub struct RefWithFlag<'a, T: 'a> {
  ptr_and_bit: usize
}
```

## My Takeaways

### Casting Multiple Times

I find the following line of code interesting in that we cast multiple times to convert the reference into a `usize` via an intermediate conversion to a raw pointer. While it is not mind-blowing, it is out of my normal experience (i.e. casting an Integer as a Float).

```
ptr_and_bit: ptr as *const T as usize
```

### Data Alignment

I learned about [data structure alignment](https://en.wikipedia.org/wiki/Data_structure_alignment) and that many types must be placed at even addresses in memory. I'm not embarrassed to admit that I had to think why we can assume the least significant bit of an even address must be 0 (hint: `even_number % 0 == 0`).
