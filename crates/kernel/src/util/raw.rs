pub struct Raw<T>(*const T);
impl<T> Raw<T> {
    pub fn from(value: &T) -> Raw<T> {
        Raw(value)
    }

    pub fn get(&self) -> Option<&T> {
        if self.0.is_null() {
            None
        } else {
            unsafe { Some(&*self.0) }
        }
    }
    pub fn into(self) -> Option<T> {
        if self.0.is_null() {
            None
        } else {
            let value = unsafe {
                if self.0.is_aligned() {
                    std::ptr::read_volatile(self.0)
                } else {
                    std::ptr::read_unaligned(self.0)
                }
            };
            Some(value)
        }
    }
    pub fn into_raw(self) -> *const T {
        let ptr = self.0;
        std::mem::forget(self);
        ptr
    }
    pub fn from_raw(ptr: *const T) -> Raw<T> {
        Raw(ptr)
    }
    pub fn as_raw(&self) -> *const T {
        self.0
    }
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    pub fn null() -> Raw<T> {
        Raw(std::ptr::null())
    }
}
impl<T> Default for Raw<T> {
    fn default() -> Self {
        Raw::null()
    }
}
impl<T> Clone for Raw<T> {
    fn clone(&self) -> Self {
        Raw(self.0)
    }
}
impl<T> AsRef<T> for Raw<T> {
    fn as_ref(&self) -> &T {
        self.get().unwrap()
    }
}
unsafe impl<T> Send for Raw<T> {}
unsafe impl<T> Sync for Raw<T> {}
