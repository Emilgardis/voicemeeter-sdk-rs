// from https://github.com/Michael-F-Bryan/ffi_helpers/blob/993b7dd01d74fdc81bf7aa2ea280f1e600e1840b/src/split.rs
// and https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
use std::ffi::c_void;

/// Splits a closure into its data part and its code part, allowing it to be
/// used as a callback by FFI code.
///
/// # Examples
///
/// ```rust
/// use std::ffi::c_void;
///
/// let mut total = 0;
///
/// // let's define a closure which will update a total and return its new value
/// let mut some_closure = |n: usize| { total += n; total };
///
/// // the callback the C function is expecting
/// type Callback = unsafe extern "C" fn(*mut c_void, usize) -> usize;
///
/// // pretend this is some C function which will periodically call a callback,
/// // passing along a user-provided pointer for state.
/// unsafe fn some_c_function(max_value: usize, cb: Callback, user_data: *mut c_void) {
///     for i in 0..max_value {
///         let got = cb(user_data, i);
///         println!("iteration: {}, total: {}", i, got);
///     }
/// }
///
/// unsafe {
///     // split the closure into its state section and the code section
///     let (state, callback) = ffi_helpers::split_closure(&mut some_closure);
///
///     // then pass it to the C function
///     some_c_function(42, callback, state);
/// }
///
/// assert_eq!(total, (0..42).sum());
/// ```
///
/// # Safety
///
///   - The returned function can only be called with the returned pointer, or a
///     pointer to another `C` closure.
///
///   - Such call is only valid within the `'lifetime` of the borrow over the
///     `closure`, during which such pointer cannot be aliased (_e.g._, no
///     concurrent calls to the closure).
///
///   - The call must be performed within the same thread, unless `C` (_i.e._,
///     the environment captured by the closure) is `Send`.
pub fn split_closure<'lifetime, C, Args>(closure: &'lifetime mut C) -> (*mut c_void, C::Trampoline)
where
    C: Split<Args>,
{
    (closure as *mut C as *mut c_void, C::TRAMPOLINE)
}

use private::Sealed;
mod private {
    pub trait Sealed<Args> {}
}

/// A helper trait used by [`split_closure()`] to get a trampoline function
/// which will invoke the closure.
///
/// This trait is automatically implemented for any `FnMut()` callable, you
/// shouldn't implement it yourself.
pub trait Split<Args>: Sealed<Args> {
    type Trampoline;

    const TRAMPOLINE: Self::Trampoline;
}

macro_rules! impl_split {
    ($( $outer:ident ),* ; $( $inner:ident ),*) => {
        impl<Func, Ret, $($outer),*> Sealed<($( $outer, )*)> for Func
        where
            Func: FnMut($($outer),*) -> Ret,
        {}
        impl<Func, Ret, $($outer),*> Split<($( $outer, )*)> for Func
        where
            Func: FnMut($($outer),*) -> Ret,
        {
            type Trampoline = unsafe extern "C" fn(*mut c_void, $($outer),*) -> Ret;

            const TRAMPOLINE: Self::Trampoline = {
                // declare a trampoline function which will turn our pointer
                // back into an `F` and invoke it

                // Note: we're deliberately using `$inner` to generate an ident
                // for the argument
                #[allow(non_snake_case)]
                unsafe extern "C" fn trampoline<T, Ret_, $( $inner ),*>(ptr: *mut c_void, $($inner: $inner),*) -> Ret_
                where
                    T: FnMut($($inner),*) -> Ret_,
                {
                    debug_assert!(!ptr.is_null());
                    let callback: &mut T;
                    unsafe {
                        callback = &mut *(ptr as *mut T);
                    }
                    callback($($inner),*)
                }

                trampoline::<Func, Ret, $($outer,)*>
            };
        }
    };
}

impl_split!(;);
impl_split!(A; A);
impl_split!(A, B; A, B);
impl_split!(A, B, C; A, B, C);
impl_split!(A, B, C, D; A, B, C, D);
impl_split!(A, B, C, D, E; A, B, C, D, E);
impl_split!(A, B, C, D, E, F; A, B, C, D, E, F);
impl_split!(A, B, C, D, E, F, G; A, B, C, D, E, F, G);
impl_split!(A, B, C, D, E, F, G, H; A, B, C, D, E, F, G, H);
impl_split!(A, B, C, D, E, F, G, H, I; A, B, C, D, E, F, G, H, I);
impl_split!(A, B, C, D, E, F, G, H, I, K; A, B, C, D, E, F, G, H, I, K);
impl_split!(A, B, C, D, E, F, G, H, I, K, L; A, B, C, D, E, F, G, H, I, K, L);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M; A, B, C, D, E, F, G, H, I, K, L, M);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M, N; A, B, C, D, E, F, G, H, I, K, L, M, N);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M, N, O; A, B, C, D, E, F, G, H, I, K, L, M, N, O);