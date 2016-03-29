
/// General matrix multiply kernel
pub trait GemmKernel {
    type Elem: Copy;

    /// align inputs to this
    ///
    /// NOTE: Not yet used.
    fn align_to() -> usize;

    /// Kernel rows
    fn mr() -> usize;
    /// Kernel cols
    fn nr() -> usize;

    fn nc() -> usize;
    fn kc() -> usize;
    fn mc() -> usize;

    /// Matrix multiplication kernel
    ///
    /// This does the matrix multiplication:
    ///
    /// C := alpha * A * B + beta * C
    ///
    /// + `k`: length of data in a, b
    /// + a, b are packed
    /// + c has general strides
    /// + rsc: row stride of c
    /// + csc: col stride of c
    /// + if `beta` is `0.`, then c does not need to be initialized
    unsafe fn kernel(
        k: usize,
        alpha: Self::Elem,
        a: *const Self::Elem,
        b: *const Self::Elem,
        beta: Self::Elem,
        c: *mut Self::Elem, rsc: isize, csc: isize);

    /// Masked (partial) kernel
    unsafe fn kernel_masked(
        k: usize,
        alpha: Self::Elem,
        a: *const Self::Elem,
        b: *const Self::Elem,
        beta: Self::Elem,
        c: *mut Self::Elem, rsc: isize, csc: isize,
        nr_: usize, mr_: usize);
}

pub trait Element : Copy {
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn scale_by(&mut self, x: Self);
}

impl Element for f32 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn is_zero(&self) -> bool { *self == 0. }
    fn scale_by(&mut self, x: Self) {
        *self *= x;
    }
}

impl Element for f64 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn is_zero(&self) -> bool { *self == 0. }
    fn scale_by(&mut self, x: Self) {
        *self *= x;
    }
}
