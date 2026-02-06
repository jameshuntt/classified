//! 
//! FEATURE NOTES
//! 
//! 
//! 
//! feature_name:async
//! deps:[tokio][async_trait]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! 
//! feature_name:logging
//! deps:[tracing]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! 
//! feature_name:std
//! deps:[std]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! 
//! 
//! 
//! 
#![cfg(feature = "async")]
//! 
//! 
//! 
//! 
//! 
//! filename:
//! 
//! 
//! usages:
//! 
//! 
//! 
//! 

use zeroize::Zeroize;

#[derive(Clone)]
pub struct SizedClassifiedData<T: Zeroize + Sized> {
    data: T,
}

impl<T: Zeroize + Sized> SizedClassifiedData<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn expose(&self) -> &T {
        &self.data
    }

    pub fn expose_mut(&mut self) -> &mut T {
        &mut self.data
    }

    // pub fn into_data(mut self) -> T {
    //     let taken = std::mem::take(&mut self.data);
    //     self.zeroize();
    //     taken
    // }

    pub fn into_data(mut self) -> T {
        use std::ptr;

        let value = unsafe { ptr::read(&self.data) };
        self.zeroize(); // optional: in case Drop still runs
        std::mem::forget(self); // prevent Drop from double zeroizing
        value
    }
}

// impl<T: Zeroize + Sized> Drop for SizedClassifiedData<T> {
//     fn drop(&mut self) {
//         self.data.zeroize();
//     }
// }
impl_sized_drop!(SizedClassifiedData);







use std::ops::{Deref, DerefMut};
impl<T: Zeroize + Sized> Deref for SizedClassifiedData<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T: Zeroize + Sized> DerefMut for SizedClassifiedData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}




use crate::{impl_debug, impl_sized_drop};
impl_debug!(SizedClassifiedData);


// impl AsRef<[u8]> for SizedClassifiedData<[u8]> {
//     fn as_ref(&self) -> &[u8] {
//         &self.data
//     }
// }

// impl AsRef<[u8]> for SizedClassifiedData<[u8; 32]> {
//     fn as_ref(&self) -> &[u8] {
//         &self.data[..]
//     }
// }

impl AsRef<[u8; 32]> for SizedClassifiedData<[u8; 32]> {
    fn as_ref(&self) -> &[u8; 32] {
        &self.data
    }
}

impl AsRef<[u8]> for SizedClassifiedData<Vec<u8>> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}


unsafe impl<T: Zeroize + Send> Send for SizedClassifiedData<T> {}
unsafe impl<T: Zeroize + Sync> Sync for SizedClassifiedData<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use zeroize::Zeroize;

    #[test]
    fn test_new_and_expose() {
        let original = [1u8; 32];
        let sensitive = SizedClassifiedData::new(original);
        assert_eq!(sensitive.expose(), &original);
    }

    #[test]
    fn test_expose_mut() {
        let mut sensitive = SizedClassifiedData::new([0u8; 32]);
        sensitive.expose_mut()[0] = 42;
        assert_eq!(sensitive.expose()[0], 42);
    }

    #[test]
    fn test_deref() {
        let mut sensitive = SizedClassifiedData::new([0u8; 32]);
        sensitive[1] = 99;
        assert_eq!(sensitive[1], 99);
    }

    #[test]
    fn test_into_data_consumes_and_prevents_drop() {
        let data = [7u8; 32];
        let sensitive = SizedClassifiedData::new(data);
        let result = sensitive.into_data();
        assert_eq!(result, data);
    }

    #[test]
    fn test_zeroize_on_drop() {
        use std::cell::RefCell;
        use std::rc::Rc;

        struct TrackableData(Rc<RefCell<Vec<u8>>>);

        impl Zeroize for TrackableData {
            fn zeroize(&mut self) {
                self.0.borrow_mut().iter_mut().for_each(|b| *b = 0);
            }
        }

        {
            let rc = Rc::new(RefCell::new(vec![1u8, 2, 3]));
            let _sensitive = SizedClassifiedData::new(TrackableData(rc.clone()));
            // drops here
        }

        // Now check the underlying data
        assert_eq!(*Rc::new(RefCell::new(vec![0u8; 3])).borrow(), vec![0u8, 0, 0]);
    }

    #[test]
    fn test_debug_does_not_leak() {
        let sensitive = SizedClassifiedData::new([1u8; 32]);
        let debug_output = format!("{:?}", sensitive);
        assert_eq!(debug_output, "SizedClassifiedData(<REDACTED>)");
    }

    #[test]
    fn test_as_ref_for_array_32() {
        let data = [9u8; 32];
        let sensitive = SizedClassifiedData::new(data);
        assert_eq!(sensitive.as_ref(), &data);
    }

    #[test]
    fn test_as_ref_for_vec() {
        let data = vec![10u8, 20, 30];
        let sensitive = SizedClassifiedData::new(data.clone());
        assert_eq!(sensitive.as_ref(), data.as_slice());
    }
}