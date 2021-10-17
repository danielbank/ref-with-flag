use std::marker::PhantomData;
use std::mem::align_of;

/// A `&T` and a bool, wrapped up in a single word.
/// The type `T` must require at least two-byte alignment.

pub struct RefWithFlag<'a, T: 'a> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T>, // occupies no space
}

impl<'a, T: 'a> RefWithFlag<'a, T> {
    pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
        assert!(align_of::<T>() % 2 == 0);
        RefWithFlag {
            ptr_and_bit: ptr as *const T as usize | flag as usize,
            behaves_like: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &'a T {
        unsafe {
            let ptr = (self.ptr_and_bit & !1) as *const T;
            &*ptr
        }
    }

    pub fn get_flag(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }

    pub fn set_flag(&mut self, flag: bool) {
        let ptr = (self.ptr_and_bit & !1) as *const T;
        self.ptr_and_bit = ptr as usize | flag as usize;
    }
}
