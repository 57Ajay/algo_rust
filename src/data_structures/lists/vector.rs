use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::ptr::NonNull;
use std::slice;

#[allow(dead_code)]
pub struct MyVec<T> {
    ptr: ptr::NonNull<T>,
    pub cap: usize,
    pub len: usize,

    // Tell the compiler: "I conceptually own values of type T"
    // This is important for the Drop checker.
    _marker: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "Not ready for ZST yet");

        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            let dst_ptr = self.ptr.as_ptr();
            let target_ptr = dst_ptr.add(self.len);
            ptr::write(target_ptr, elem);
        }

        self.len += 1;
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            let new_cap = 4;
            let layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, layout)
        } else {
            let new_cap = self.cap * 2;
            let layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, layout)
        };

        // ensuring allocation isen't too big
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = unsafe {
            if self.cap == 0 {
                alloc::alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                let old_ptr = self.ptr.as_ptr() as *mut u8;

                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap == 0 {
            return;
        }

        unsafe {
            for i in 0..self.len {
                let elem_ptr = self.ptr.as_ptr().add(i);
                ptr::drop_in_place(elem_ptr);
            }

            let layout = Layout::array::<T>(self.cap).unwrap();
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = self.ptr.as_ptr();
            slice::from_raw_parts(ptr, self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = self.ptr.as_ptr();
            slice::from_raw_parts_mut(ptr, self.len)
        }
    }
}
