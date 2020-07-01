use std::cell::UnsafeCell;

// 由于 UnsafeCell 实现了 !Sync trait，故 Cell 也实现了 !Sync trait
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no one else is concurrently mutating self.value
        //         (because !Sync)
        // SAFETY: we know we're not invalidating any references, because
        //         we never give any out
        unsafe {
            // UnsafeCell<T> 的 get 方法返回一个底层数据的可变引用
            *self.value.get() = value
        }
    }

    // 由于 get 操作的返回值 T 实现了 Copy marker，也就是说，返回的是值不是引用，
    // 这就保证了，在 get 之后调用 set 是安全的。如果 get 返回一个引用，则之后再
    // 调用 set，可能将 get 返回值所指向的内容清空，导致 get 的返回值变为空指针
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no one else is modifying this value, since only
        // this thread can mutate (!Sync)
        unsafe {
            *self.value.get()
        }
    }
}