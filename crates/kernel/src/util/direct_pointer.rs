pub struct DirectPointer<T>(pub *mut T);
impl<T> Drop for DirectPointer<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            self.0 = std::ptr::null_mut();
        }
    }
}
impl<T> DirectPointer<T> {
    pub fn new(value: &mut T) -> DirectPointer<T> {
        DirectPointer(value)
    }
    pub fn get_mut(&self) -> Option<&mut T> {
        if self.0.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.0) }
        }
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
}
unsafe impl<T> Send for DirectPointer<T> {}
unsafe impl<T> Sync for DirectPointer<T> {}
