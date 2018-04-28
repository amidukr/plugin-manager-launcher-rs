use std::cell::RefCell;
use std::cell::RefMut;

use std::sync::Mutex;
use std::sync::MutexGuard;

use std::ops::Deref;
use std::ops::DerefMut;

use std::fmt::{self, Debug, Display};

pub struct LockError {}

pub trait WriteGuard<'a, T: ?Sized + 'a> : ReadGuard<'a, T>{
    fn take_mut(&mut self) -> &mut T;
}

pub trait ReadGuard<'a, T: ?Sized + 'a> {
    fn take(&self) -> &T;
}

pub trait WriteContainer<T: ?Sized> : ReadContainer<T> {
    fn try_write_lock<'a>(&self) -> Result<WriteGuardBox<'a, T>, LockError>;
    fn write_lock<'a>(&self) -> WriteGuardBox<'a, T>;
}

pub trait ReadContainer<T: ?Sized> {
    fn try_read_lock(&self) -> Result<ReadGuardBox<T>, LockError>;
    fn read_lock(&self) -> ReadGuardBox<T>;
}

/*pub trait WriteContainerConcrete<T: ?Sized> {
    fn try_write_lock_sized<'a, W: WriteGuard<'a, T>>(&self) -> Result<W, LockError>;
    fn write_lock_sized<'a, W: WriteGuard<'a, T>>(&self) -> W;
}*/

pub trait ReadContainerConcrete<'b, T: ?Sized + 'b, R: ReadGuard<'b, T>> {
    //fn try_read_lock_sized<'a, R: ReadGuard<'a, T>>(&self) -> Result<R, LockError>;
    //fn read_lock_sized<'a>(&'a self) -> ReadGuard<'a, T>;
    fn read_lock_sized<'a: 'b>(&'a self) -> R;
}

pub struct ReadGuardBox<'a, T: ?Sized + 'a> {
    read_guard: Box<ReadGuard<'a, T>>
}

pub struct WriteGuardBox<'a, T: ?Sized + 'a> {
    write_guard: Box<WriteGuard<'a, T>>
}

impl <'a, T: 'a> Deref for ReadGuardBox<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        return self.read_guard.take();
    }
}

impl <'a, T: 'a> Deref for WriteGuardBox<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        return self.write_guard.take();
    }
}

impl <'a, T: 'a> DerefMut for WriteGuardBox<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        return self.write_guard.take_mut();
    }
}

impl Debug for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LockError").finish()
    }
}

impl Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("already locked", f)
    }
}

impl <'a, T: ?Sized> WriteGuardBox<'a, T> {
    pub fn new(write_guard: Box<WriteGuard<'a, T>>) -> WriteGuardBox<'a, T> {
        return WriteGuardBox{
            write_guard: write_guard
        }
    }
}

impl <'a, T: ?Sized + 'a> ReadGuardBox<'a, T> {
    pub fn new(read_guard: Box<ReadGuard<'a, T>>) -> ReadGuardBox<'a, T> {
        return ReadGuardBox{
            read_guard: read_guard
        }
    }

    pub fn new_box<R: ReadGuard<'a, T> + 'static>(read_guard: R) -> ReadGuardBox<'a, T> {
        return ReadGuardBox::new(Box::new(read_guard));
    }
}
