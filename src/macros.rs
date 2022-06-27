#[macro_export]
macro_rules! c_str {
    ($string: expr) => { concat!($string, "\0") }
}

#[macro_export]
macro_rules! cast {
    (mut $address: expr, $type: ident) => {
        $address as *mut $type
    };
    ($address: expr, $type: ident) => {
        $address as *const $type
    };
}

#[macro_export]
macro_rules! make_fn {
    ($address:expr) => {
        unsafe { std::mem::transmute::<*const usize, fn()>($address as *const usize) }
    };
    ($address:expr, $returntype:ty) => {
        unsafe { std::mem::transmute::<*const usize, fn() -> $returntype>($address as *const usize) }
    };
    ($address:expr, $returntype:ty, $($argument:ty), *) => {
        unsafe { std::mem::transmute::<*const usize, fn($($argument,)*) -> $returntype>($address as *const usize) }
    };
}