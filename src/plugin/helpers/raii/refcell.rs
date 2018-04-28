use std::cell::RefCell;

use plugin::api::raii::*;

use std::marker::PhantomData;

pub struct RefCellReadGuard<'a, T: ?Sized + 'a> {
    pd: PhantomData<&'a T>
}

pub struct RefCellWriteGuard<'a, T: ?Sized + 'a> {
    pd: PhantomData<&'a T>
}

pub struct RefCellContainer<T: ?Sized> {
    ref_cell: RefCell<T>
}

impl <'a, T: ?Sized + 'a> ReadGuard<'a, T> for RefCellReadGuard<'a, T> {
    #[inline]
    fn take(&self) -> &T {panic!("test");}
}

impl <'a, T: ?Sized + 'a> ReadGuard<'a, T> for RefCellWriteGuard<'a, T> {
    #[inline]
    fn take(&self) -> &T {panic!("test");}
}

/*impl <'a, T: ?Sized + 'a> WriteGuard<'a, T> for RefCellWriteGuard<'a, T> {
    #[inline]
    fn take_mut(&mut self) -> &mut T {panic!("test");}
}*/

impl <T: ?Sized> ReadContainer<T> for RefCellContainer<T> {
    fn try_read_lock(&self) -> Result<ReadGuardBox<T>, LockError> {
        //return ReadGuardBox::new(Box::new(self.try_read_lock_sized()))?;
        panic!("sdfsdf");
    }
    fn read_lock<'a>(&'a self) -> ReadGuardBox<'a, T> {
        //let rcrg:RefCellReadGuard<'a, T> = self.read_lock_sized();
        //let rcrg_box: Box<ReadGuard<'a, T>> = Box::new(rcrg);
        //let rcrg_box:ReadGuardBox<'a, T> = ReadGuardBox::new_box(rcrg);
        //return ReadGuardBox::new(rcrg_box);
        panic!("sdfsd");
        return ReadGuardBox::new_box(self.read_lock_sized());
    } 
}

/*impl <T: ?Sized> WriteContainer<T> for RefCellContainer<T> {
    fn try_write_lock(&self) -> Result<WriteGuardBox<T>, LockError> {
        
    }

    fn write_lock(&self) -> WriteGuardBox<T>;
}*/

impl <T: ?Sized> RefCellContainer<T> {
    //fn try_write_lock_sized<>(&self) -> Result<RefCellWriteGuard<T>, LockError> {panic!("test");}
    //fn write_lock_sized(&self) -> RefCellWriteGuard<T> {panic!("test");}

    fn try_read_lock_sized<'a>(&'a self) -> Result<RefCellReadGuard<'a, T>, LockError> {panic!("test");}
    fn read_lock_sized<'a>(&'a self) -> RefCellReadGuard<'a, T> {panic!("test");}
}

unsafe impl <'a, T> Send for RefCellReadGuard<'a, T> {}