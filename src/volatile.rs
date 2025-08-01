#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T> Volatile<T> {
    pub fn read(&mut self) -> T {
        unsafe { (&mut self.0 as *mut T).read_volatile() }
    }

    pub fn write(&mut self, val: T) {
        unsafe { (&mut self.0 as *mut T).write_volatile(val) }
    }
}
