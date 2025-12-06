pub use registry_internal;
pub use registry_internal::{list, register_plugin};

#[macro_export]
macro_rules! register {
    ($target:path) => {
        $crate::registry_internal::register!($crate, $target);
    };
}
