//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:zeroizing_guard.rs
//! description:
//! usages:none in crate yet
//! 
//! ----------------------------------------------
//! FEATURE NOTES --------------------------------
//! 
//! feature_name:async
//! deps:[zeroize]
//! scope:[impl ThreadPoolManager]
//! corpus:true
//! 
//! ----------------------------------------------
//! CORPUS FEATURES ------------------------------
//! 
//! [no_std]



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
