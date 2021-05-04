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

#[derive(Debug)]
pub struct Field<R: RegisterValue, T> {
    mask: R::Int,
    offset: usize,
    _reg: PhantomData<R>,
    _type: PhantomData<T>,
}

impl<R: RegisterValue, T> Field<R, T> {
    /// Get the raw mask
    #[inline]
    pub fn mask(self) -> R::Int {
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
    pub const unsafe fn from_raw(mask: R::Int, offset: usize) -> Field<R, T> {
        Field {
            mask,
            offset,
            _reg: PhantomData,
            _type: PhantomData,
        }
    }
}

impl<R: RegisterValue, T> Clone for Field<R, T> {
    #[inline]
    fn clone(&self) -> Field<R, T> {
        Field {
            mask: self.mask,
            offset: self.offset,
            _reg: PhantomData,
            _type: PhantomData,
        }
    }
}

impl<R: RegisterValue, T> Copy for Field<R, T> {}

impl<R: RegisterValue, T: Into<Fields<R>> + Copy, U> PartialEq<T> for Field<R, U> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let other: Fields<R> = (*other).into();
        self.mask == other.mask()
    }
}

impl<R: RegisterValue, T> Eq for Field<R, T> {}

impl<R: RegisterValue, T> From<Field<R, T>> for Fields<R, ()> {
    #[inline]
    fn from(field: Field<R, T>) -> Fields<R, ()> {
        unsafe { Fields::from_raw(field.mask) }
    }
}

impl<R: RegisterValue, T: MayToggle<Toggle = Toggle>> From<Field<R, T>> for Fields<R, Toggle> {
    #[inline]
    fn from(field: Field<R, T>) -> Fields<R, Toggle> {
        unsafe { Fields::from_raw(field.mask) }
    }
}

impl<R: RegisterValue, T: MayToggle> MayToggle for Field<R, T> {
    type Toggle = T::Toggle;
}

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitOr<T> for Field<R, U>
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

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitAnd<T> for Field<R, U>
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

impl<R: RegisterValue, T: Into<Fields<R>>, U: MayToggle> BitXor<T> for Field<R, U>
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
