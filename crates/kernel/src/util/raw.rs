pub struct Raw<T>(*const T);
impl<T> Raw<T> {
    pub fn get(&self) -> Option<&T> {
        if self.0.is_null() {
            None
        } else {
            unsafe { Some(&*self.0) }
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
impl<'a, T> Into<Option<&'a T>> for Raw<T> {
    fn into(self) -> Option<&'a T> {
        if self.0.is_null() {
            None
        } else {
            Some(unsafe { &*self.0 })
        }
    }
}
impl<T> From<*const T> for Raw<T> {
    fn from(value: *const T) -> Self {
        Raw(value)
    }
}
impl<T> From<Option<T>> for Raw<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Raw::from(&value),
            None => Raw::null(),
        }
    }
}
impl<T> From<&T> for Raw<T> {
    fn from(value: &T) -> Raw<T> {
        Raw(value)
    }
}

impl<T> Into<*const T> for Raw<T> {
    fn into(self) -> *const T {
        self.0
    }
}
impl<T> std::fmt::Pointer for Raw<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}
impl<T> std::fmt::Debug for Raw<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}

unsafe impl<T> Send for Raw<T> {}
unsafe impl<T> Sync for Raw<T> {}
