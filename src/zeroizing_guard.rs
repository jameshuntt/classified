//! 
//! FEATURE NOTES
//! 
//! 
//! 
//! feature_name:default
//! deps:[zeroize]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! 
//! 
//! 
//! 
//! corpus features
//! #![cfg(feature = "")]
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

pub struct ZeroizingGuard<'a, T: Zeroize> {
    pub data: &'a mut T,
    active: bool,
}

impl<'a, T: Zeroize> ZeroizingGuard<'a, T> {
    pub fn new(data: &'a mut T) -> Self {
        Self { data, active: true }
    }

    pub fn cancel(&mut self) {
        self.active = false;
    }
}

impl<'a, T: Zeroize> Drop for ZeroizingGuard<'a, T> {
    fn drop(&mut self) {
        if self.active {
            self.data.zeroize();
        }
    }
}
