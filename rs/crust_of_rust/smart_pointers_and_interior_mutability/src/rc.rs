use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

// 为什么不直接在 Rc<T> 中添加 refcount 成员？
// 如果 refcount 加在 Rc<T> 中，则每次调用 clone 都会创建一个该 Rc 自己的 count，
// 这就没办法知道到底什么时候计数会变为 0。
// 相反，refcount 应该是在所有的 Rc<T> 副本之间共享的，因此需要定义一个额外的 RcInner
// 来存储所有需要在 Rc<T> 之间共享的值
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

// 由于 NonNull 实现了 !Send，因此 Rc<T> 也是 !Send
pub struct Rc<T> {
    // 不能用 Box<T>，因为如果用 Box<T>，每次复制都要从堆上分配内存
    // 使用 NonNull 是因为后面的 Box::from_raw 方法需要一个 *mut 的裸指针，而 inner 最好定义为
    // *const 裸指针，所以选择了 NonNull
    inner: NonNull<RcInner<T>>,
    // 需要添加该 marker 是为了实现 rust 的 drop check：即，当需要删除 Rc<T> 时，
    // 注意 T 也是属于 Rc 的，要对 T 也进行 drop check。如果此时 T 已经被提前删除了，
    // 则该程序在编译时应该报错
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        // 需要使用 Box 在堆上分配内存
        let inner = Box::new(RcInner{
            value,
            refcount: Cell::new(1),
        });

        // 返回时不能返回 Rc{ inner: &*inner }，因为在 new 函数返回时，由于已经离开了
        // 作用域，在作用域中创建的 Box 也会被清理，分配的内存会被释放
        // 因此需要使用 Box::into_raw 来获取裸指针，避免内存被释放
        Rc {
            // SAFETY: Box does not give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    // 注意，这里并没有要求 T 实现 Clone，因为我们的 clone 操作
    // 实际上是增加引用计数，并不是实际拷贝值
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc
        // goes away, we have an Rc, therefore the Box has not been deallocated,
        // so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // 为什么要先 drop(inner)，再清除 Box?
            // 这里要做的是回收由 Box 在堆上分配的内存，使用的方法是：通过 Box::from_raw() 将裸指针的所有权
            // 交还给 Box，然后把 Box 赋值给 let _，表示丢弃该值，由此通过智能指针的自动回收机制完成了堆上内存
            // 的回收。
            // inner 是一个指向堆上已分配内存的引用，通过 drop(inner) 只是删除了这个引用，没有清理堆上的内存。
            // 先调用 drop 是因为在 Box::from_raw() 返回后，由于生成的 Box 的值被丢弃，导致内存清理，这时候
            // 指向该内存地址的指针已经失效。防止在此之后再次调用 inner 导致出错，所以先删除了 inner
            drop(inner);
            // SAFETY: we are the only Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc, and no reference to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are Rcs, so don't drop the Box!
            inner.refcount.set(c - 1);
        }
    }
}
