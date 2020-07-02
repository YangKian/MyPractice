use std::cell::UnsafeCell;
use crate::cell::Cell;

enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    // 使用 Cell<T> 封装 RefState 的原因：在 borrow 和 borrow_mut 两个 API 中，
    // 传入的都是不可变参数 &self, 然而代码的逻辑要求对不可变引用进行修改，所以使用
    // Cell<T> 进行封装
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    // borrow 和 borrow_mut 不能返回 Option<&T> 或者 Option<&mut T> 的原因：
    // 我们只实现了在返回引用时增加计数，但是在引用销毁时减少计数没法实现
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref::new(self))
            },
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref::new(self))
            },
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state
            // Shared and Exclusive will go to the else branch
            Some(RefMut::new(self))
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Ref<T> {
    pub fn new(value: T) -> Self {
        Self {
            refcell: value,
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exclusive references have been given out.
        // because once it is given out, state is set to Shared. so dereferencing into a
        // shared reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> RefMut<T> {
    pub fn new(value: T) -> Self {
        Self {
            refcell: value,
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: see safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to Exclusive, so no future references
        // are given out. so we have an exclusive lease on the inner value, so mutably
        // dereferencing is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}