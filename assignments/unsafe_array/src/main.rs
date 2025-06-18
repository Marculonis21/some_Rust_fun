// use std::ptr;
// use std::slice;
// use std::ops::{Index, IndexMut, Deref, DerefMut};

// pub struct Array<T> {
//     ptr: *mut T,
//     size: usize,
//     destructor: fn(*mut T),
// }

// impl<T: Default> Array<T> {
//     // Unsafe constructor that takes ownership of foreign memory
//     pub unsafe fn new(ptr: *mut T, size: usize, destructor: fn(*mut T)) -> Self {

//         unsafe {
//             for i in 0..size {
//                 let slot = ptr.add(i);
//                 ptr::write(slot, T::default()); // Initialize memory to T::default()
//             }
//         }

//         Array {
//             ptr,
//             size,
//             destructor,
//         }
//     }
// }

// impl<T> Drop for Array<T> {
//     fn drop(&mut self) {
//         (self.destructor)(self.ptr);
//     }
// }

// // For &[T] and &mut [T] deref 
// impl<T> Deref for Array<T> {
//     type Target = [T];

//     fn deref(&self) -> &Self::Target {
//         unsafe { slice::from_raw_parts(self.ptr, self.size) }
//     }
// }

// impl<T> DerefMut for Array<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         unsafe { slice::from_raw_parts_mut(self.ptr, self.size) }
//     }
// }

// // Read indexing
// impl<T> Index<usize> for Array<T> {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         if index >= self.size {
//             panic!("Index out of bounds");
//         }

//         unsafe { &*self.ptr.add(index) }
//     }
// }

// // Write indexing
// impl<T> IndexMut<usize> for Array<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         if index >= self.size {
//             panic!("Index out of bounds");
//         }
//         unsafe { &mut *self.ptr.add(index) }
//     }
// }

// // Iteration over &Array<T>
// impl<'a, T> IntoIterator for &'a Array<T> {
//     type Item = &'a T;
//     type IntoIter = slice::Iter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.deref().iter()
//     }
// }

// // Iteration over &mut Array<T>
// impl<'a, T> IntoIterator for &'a mut Array<T> {
//     type Item = &'a mut T;
//     type IntoIter = slice::IterMut<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.deref_mut().iter_mut()
//     }
// }

// // Consuming iterator — gives ownership of T if desired
// impl<T> IntoIterator for Array<T> {
//     type Item = T;
//     type IntoIter = ArrayIntoIter<T>;

//     fn into_iter(self) -> Self::IntoIter {
//         let iter = unsafe {
//             let vec = Vec::from_raw_parts(self.ptr, self.size, self.size);
//             std::mem::forget(self); // We take over ownership, avoid Drop
//             vec.into_iter()
//         };

//         ArrayIntoIter { inner: iter }
//     }
// }

// pub struct ArrayIntoIter<T> {
//     inner: std::vec::IntoIter<T>,
// }

// impl<T> Iterator for ArrayIntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next()
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.inner.size_hint()
//     }
// }

// impl<T> Drop for ArrayIntoIter<T> {
//     fn drop(&mut self) {

//     }
// }

mod solution;
use solution::Array;

#[derive(Default, Debug)]
struct Item {
    value: i32,
}

const SIZE: usize = 5;

fn my_destructor(ptr: *mut Item) {
    unsafe {
        for i in 0..SIZE {
            std::ptr::drop_in_place(ptr.add(i));
        }

        let layout = std::alloc::Layout::array::<Item>(SIZE).unwrap();
        std::alloc::dealloc(ptr as *mut u8, layout);
        println!("\nCustom destructor called!");
    }
}

fn main() {
    let layout = std::alloc::Layout::array::<Item>(SIZE).unwrap();

    unsafe {
        // Allocate uninitialized memory for `size` Items
        let raw_ptr = std::alloc::alloc(layout) as *mut Item;

        // Create the array with a destructor that drops + deallocates
        let mut array = Array::new(raw_ptr, SIZE, my_destructor);

        // Write values using indexing
        for i in 0..SIZE {
            array[i].value = (i as i32) * 10;
        }

        // Read values using indexing
        println!("Testing indexing:");
        for i in 0..SIZE {
            println!("array[{}] = {:?}", i, array[i]);
        }

        // Iterate immutably
        println!("\nImmutable iteration:");
        for item in &array {
            println!("{:?}", item);
        }

        // Iterate mutably and modify items
        println!("\nMutable iteration:");
        for item in &mut array {
            println!("Pre mut {:?}",item);
            item.value += 1;
            println!("Post mut {:?}",item);
        }

        // Iterate mutably and modify items
        // println!("\nConsuming iteration: (no destructor called)");
        // for item in array {
        //     println!("{:?}", item)
        // }

        // Dropping `array` will invoke the custom destructor
    }
}
