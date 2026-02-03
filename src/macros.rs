#[macro_export]
macro_rules! impl_deref {
    ($type_name:ident) => {
        /// Enables dereferencing to the inner value for ergonomic use.
        ///
        /// ⚠️ This should be used carefully, especially with traits that
        /// could expose the inner secret (e.g., `Debug`, `Display`).
        impl<T> ::std::ops::Deref for $type_name<T>
        where
            T: $crate::zeroize::Zeroize
        {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.data.expose_secret()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_secure_eq {
    ($type_name:ident) => {
        /// Provides a semantic wrapper over `ct_eq`, returning a boolean.
        impl $crate::traits::ClassifiedEq for $type_name<Vec<u8>> {
            fn classified_eq(&self, rhs: &Self) -> bool {
                self.ct_eq(rhs).into()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_drop {
    ($type_name:ident) => {
        /// Ensures that the inner secret is zeroized before memory is freed.
        impl<T> Drop for $type_name<T> 
        where 
            T: $crate::zeroize::Zeroize 
        {
            fn drop(&mut self) {
                self.data.zeroize();
            }
        }
    };
}
#[macro_export]
macro_rules! impl_generic_drop {
    ($type_name:ident<$($gen:ident),+>, $field_name:ident) => {
        impl<$($gen),+> Drop for $type_name<$($gen),+>
        where
            $($gen: $crate::zeroize::Zeroize),+
        {
            fn drop(&mut self) {
                self.$field_name.zeroize();
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sized_drop {
    ($type_name:ident) => {
        /// Ensures that the inner secret is zeroized before memory is freed.
        impl<T> Drop for $type_name<T> 
        where 
            T: $crate::zeroize::Zeroize + Sized
        {
            fn drop(&mut self) {
                self.data.zeroize();
            }
        }
    };
}

#[macro_export]
macro_rules! impl_basic_debug {
    ($type_name:ident) => {
        /// Hides inner secrets from accidental logging or printing.
        ///
        /// Always shows `<redacted>`, regardless of the wrapped value.
        impl<T> ::std::fmt::Debug for $type_name<T>
            where T: $crate::zeroize::Zeroize
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("ClassifiedData(<REDACTED>)")
            }
        }
    };
}

#[macro_export]
macro_rules! impl_clone {
    ($type_name:ident) => {
        #[cfg(not(feature = "no-clone"))]
        /// Allows cloning of classified data only if the feature `no-clone` is not set.
        ///
        /// ⚠️ Cloning secrets can be dangerous and should only be enabled when necessary.
        impl<T> Clone for $type_name<T>
        where
            T: $crate::zeroize::Zeroize + Clone
        {
            fn clone(&self) -> Self {
                $type_name {
                    data: $crate::secrecy::SecretBox::new(
                        Box::new(self.data.expose_secret().clone())
                    ),
                }
            }
        }

        #[cfg(feature = "no-clone")]
        /// Prevents cloning of classified data when `no-clone` is enabled,
        /// enforcing strong immutability of secrets.
        impl<T> Clone for $type_name<T>
        where
            T: $crate::zeroize::Zeroize + Clone
        {
            fn clone(&self) -> Self {
                panic!("Cloning sensitive data is forbidden");
            }
        }
    }
}

#[macro_export]
macro_rules! impl_debug {
    // Default label = the type name
    ($type_name:ident) => {
        $crate::impl_debug!($type_name, stringify!($type_name));
    };

    // Custom label
    ($type_name:ident, $label:expr) => {
        impl<T> ::std::fmt::Debug for $type_name<T>
        where
            T: $crate::zeroize::Zeroize,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                // Writes: "<label>(<REDACTED>)"
                ::core::write!(f, "{}(<REDACTED>)", $label)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ct {
    // Default label = the type name
    ($type_name:ident) => {
        impl $crate::subtle::ConstantTimeEq for $type_name<Vec<u8>> {
            fn ct_eq(&self, other: &Self) -> $crate::subtle::Choice {
                self.expose().ct_eq(other.expose())
            }
        }
    };
}


/// Implements secure memory and ergonomic trait impls for secret types.
///
/// Usage:
/// ```
/// use secrecy::{SecretBox, ExposeSecret};
/// use classified::impl_secure_classified;
/// use zeroize::Zeroize;
/// use subtle::ConstantTimeEq;
/// struct MySecret<T: zeroize::Zeroize> { data: SecretBox<T> }
/// impl_secure_classified!(MySecret);
/// ```
#[macro_export]
macro_rules! impl_secure_classified {
    ($type_name:ident) => {
        $crate::impl_deref!($type_name);
        $crate::impl_drop!($type_name);
        $crate::impl_debug!($type_name);
        $crate::impl_clone!($type_name);
        $crate::impl_ct!($type_name);
        $crate::impl_secure_eq!($type_name);
        $crate::impl_expose!($type_name);
    };
}

#[macro_export]
macro_rules! impl_expose {
    ($type_name:ident) => {
        impl<T> $type_name<T>
        where
            T: $crate::zeroize::Zeroize
        {
            /// Expose a reference to the inner value.
            ///
            /// ⚠️ Use with care. This is a read-only view of sensitive data.
            #[must_use = "You must never ignore confidential data"]
            pub fn expose(&self) -> &T {
                self.data.expose_secret()
            }
        }
    };
    ($type_name:ident, $expr:expr) => {
        impl<T> $type_name<T>
        where
            T: $crate::zeroize::Zeroize
        {
            /// Expose a reference to the inner value.
            ///
            /// ⚠️ Use with care. This is a read-only view of sensitive data.
            #[must_use = "You must never ignore confidential data"]
            pub fn expose(&self) -> &T {
                ($expr)(self)
            }
        }
        
    };
}

#[macro_export]
macro_rules! impl_new {
    ($type_name:ident) => {
        impl<T> $type_name<T>
        where
            T: $crate::zeroize::Zeroize
        {
            /// Create a new classified value, wrapping the given data securely.
            ///
            /// The inner data will be zeroed on drop, and access is controlled.
            ///
            /// # Example
            /// ```
            /// use classified::classified_data::ClassifiedData;
            /// let secret = ClassifiedData::new("my-api-key".to_string());
            /// ```
            pub fn new(data: T) -> Self {
                $type_name {
                    data: SecretBox::new(Box::new(data)),
                }
            }
        }
    }
}


