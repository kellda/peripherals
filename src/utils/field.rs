use super::*;

/// A single register field
///
/// # Created by:
/// - Associated constants on register defined with the [`periph!`] or the [`register!`] macros.
///
/// # Usable with:
/// - [`Reg::field`] or [`Value::field`]: read this field to its specific type.
/// - [`Reg::toggle`] or [`Value`]` ^ `[`Field`]: toggle these fields (only for single-bit fields).
///
/// These fields be combined together with `|`, `&` and `^`, producing [`Fields`].
///
/// # Example
///
/// ```
/// use peripherals::{register, Value};
///
/// register! {
///     Register: u8 = 0b1001 {
///         BIT1: 0 = struct Bit1(bool);
///         BIT2: 1 = struct Bit2(bool);
///         TWO_BITS: 2..3 = struct TwoBits(u8);
///     }
/// }
///
/// // Obtain it from the associated constants on the register
/// assert_eq!(Register::BIT1.mask(), 0b0001);
/// assert_eq!(Register::TWO_BITS.mask(), 0b1100);
///
/// // Use it to read fields
/// let mut value = Value::reset();
/// assert_eq!(value.value(), 0b1001);
///
/// assert_eq!(value.field(Register::BIT1), Bit1(true));
/// assert_eq!(value.field(Register::BIT2), Bit2(false));
/// assert_eq!(value.field(Register::TWO_BITS), TwoBits(0b10));
///
/// // Toggle single-bit fields
/// value ^= Register::BIT1;
/// assert_eq!(value.value(), 0b1000);
/// value ^= Register::BIT2;
/// assert_eq!(value.value(), 0b1010);
/// ```

pub struct Field<R, T, I> {
    mask: I,
    offset: usize,
    _reg: PhantomData<R>,
    _type: PhantomData<T>,
}

impl<R, T, I> Field<R, T, I> {
    /// Get the raw mask
    #[inline]
    pub fn mask(self) -> I {
        self.mask
    }

    /// Get the field offset
    #[inline]
    pub fn offset(self) -> usize {
        self.offset
    }

    /// Build from raw mask
    ///
    /// # Safety
    ///
    /// You should ensure the mask is valid for the fields of the associated register.
    #[inline]
    pub const unsafe fn from_raw(mask: I, offset: usize) -> Field<R, T, I> {
        Field {
            mask,
            offset,
            _reg: PhantomData,
            _type: PhantomData,
        }
    }

    /// If this compiles, the `RegisterValue` trait can be added to the `Field` struct, and `R::Int`
    /// can be used instead of a third type parameter `I`. As of rust 1.52.0, this requires the
    /// `const_fn` feature flag.
    /// ```compile_fail
    /// const fn foo<T: Default>(t: T) -> T { t }
    /// ```
    /// ```
    /// fn foo<T: Default>(t: T) {}
    /// ```
    fn _check_const_fn() {}
}

impl<R: RegisterValue, T> Clone for Field<R, T, R::Int> {
    #[inline]
    fn clone(&self) -> Field<R, T, R::Int> {
        Field {
            mask: self.mask,
            offset: self.offset,
            _reg: PhantomData,
            _type: PhantomData,
        }
    }
}

impl<R: RegisterValue, T> Copy for Field<R, T, R::Int> {}

impl<R: RegisterValue, T> Debug for Field<R, T, R::Int> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if fmt.alternate() {
            write!(
                fmt,
                "Field<{}>(0b{:02$b})",
                R::NAME,
                self.mask,
                <R::Int as Int>::WIDTH
            )
        } else {
            write!(
                fmt,
                "Field<{}>(0x{:02$x})",
                R::NAME,
                self.mask,
                <R::Int as Int>::WIDTH / 4
            )
        }
    }
}

impl<R: RegisterValue, T: Into<Fields<R>> + Copy, U> PartialEq<T> for Field<R, U, R::Int> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let other: Fields<R> = (*other).into();
        self.mask == other.mask()
    }
}

impl<R: RegisterValue, T> Eq for Field<R, T, R::Int> {}

impl<R: RegisterValue, T> From<Field<R, T, R::Int>> for Fields<R, ()> {
    #[inline]
    fn from(field: Field<R, T, R::Int>) -> Fields<R, ()> {
        unsafe { Fields::from_raw(field.mask) }
    }
}

impl<R: RegisterValue, T: MayToggle<Toggle = Toggle>> From<Field<R, T, R::Int>>
    for Fields<R, Toggle>
{
    #[inline]
    fn from(field: Field<R, T, R::Int>) -> Fields<R, Toggle> {
        unsafe { Fields::from_raw(field.mask) }
    }
}

impl<R: RegisterValue, T: MayToggle> MayToggle for Field<R, T, R::Int> {
    type Toggle = T::Toggle;
}

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitOr<T> for Field<R, U, R::Int>
where
    T: Both<U::Toggle>,
{
    type Output = Fields<R, T::Output>;

    #[inline]
    fn bitor(self, other: T) -> Self::Output {
        let other = other.into();
        unsafe { Fields::from_raw(self.mask | other.mask()) }
    }
}

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitAnd<T> for Field<R, U, R::Int>
where
    T: Either<U::Toggle>,
{
    type Output = Fields<R, T::Output>;

    #[inline]
    fn bitand(self, other: T) -> Self::Output {
        let other = other.into();
        unsafe { Fields::from_raw(self.mask & other.mask()) }
    }
}

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitXor<T> for Field<R, U, R::Int>
where
    T: Both<U::Toggle>,
{
    type Output = Fields<R, T::Output>;

    #[inline]
    fn bitxor(self, other: T) -> Self::Output {
        let other = other.into();
        unsafe { Fields::from_raw(self.mask ^ other.mask()) }
    }
}
