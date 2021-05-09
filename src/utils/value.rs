use super::*;

/// A value read from or to be written to a register
///
/// # Created by:
/// - [`Reg::read`]: read the value from the register.
/// - `Default` or the [`Value::reset`]: the reset value of the register.
///
/// # Usable with:
/// - [`Reg::write`]: write the value to the register.
/// - [`Value`]` & `[`Fields`] extracts [`FieldValues`].
///
/// Values can be modified with the `|` and `^` operators, and well as with `|=` and `^=`.
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
/// // Obtain it with the `reset` method or by reading the register
/// let mut value = Value::<Register>::reset();
/// assert_eq!(value.value(), 0b1001);
///
/// /// Extract single field
/// assert_eq!(value.field(Register::BIT1), Bit1(true));
/// assert_eq!(value.field(Register::BIT2), Bit2(false));
/// assert_eq!(value.field(Register::TWO_BITS), TwoBits(0b10));
///
/// // Extract multiple fields
/// let fields_12 = value & (Register::BIT1 | Register::BIT2);
/// assert_eq!(fields_12.bits(), 0b001);
/// assert_eq!((value & Register::TWO_BITS).bits(), 0b1000);
///
/// // Toggle single-bit fields
/// value ^= Register::BIT1;
/// assert_eq!(value.value(), 0b1000);
/// value ^= Register::BIT1 | Register::BIT2;
/// assert_eq!(value.value(), 0b1011);
///
/// // Write back fields previously read
/// value |= fields_12;
/// assert_eq!(value.value(), 0b1001);
///
/// // Modify fields
/// value |= Bit1(false);
/// assert_eq!(value.value(), 0b1000);
/// value |= TwoBits(0b01);
/// assert_eq!(value.value(), 0b0100);
/// ```

pub struct Value<R: RegisterValue> {
    value: R::Int,
    _reg: PhantomData<R>,
}

impl<R: RegisterValue> Value<R> {
    /// Get the raw value
    #[inline]
    pub fn value(self) -> R::Int {
        self.value
    }

    /// Build from a raw value
    ///
    /// # Safety
    ///
    /// You must ensure the value is valid for the associated register.
    #[inline]
    pub unsafe fn from_raw(value: R::Int) -> Value<R> {
        Value {
            value,
            _reg: PhantomData,
        }
    }

    /// Read the given field
    ///
    /// This returns the value of a field defined with the [`periph!`] or [`register!`] macro.
    #[inline]
    pub fn field<T>(self, field: Field<R, T, R::Int>) -> T
    where
        R::Int: TryInto<T>,
        <R::Int as TryInto<T>>::Error: Debug,
    {
        ((self.value & field.mask()) >> field.offset())
            .try_into()
            .unwrap()
    }

    /// Test the given fields
    ///
    /// This returns true if the field has the value given in parameter. It can also be used with
    /// more values, combined with `|`, in which case all fields must match.
    #[inline]
    pub fn test<B: Into<FieldValues<R>>>(self, bits: B) -> bool {
        let bits = bits.into();
        self.value & bits.mask() == bits.bits()
    }

    /// Get the default / reset value
    ///
    /// This returns to the value that the register has right right after a reset or a boot.
    #[inline]
    pub fn reset() -> Value<R> {
        Value {
            value: R::RESET,
            _reg: PhantomData,
        }
    }
}

impl<R: RegisterValue> Clone for Value<R> {
    #[inline]
    fn clone(&self) -> Value<R> {
        Value {
            value: self.value,
            _reg: PhantomData,
        }
    }
}

impl<R: RegisterValue> Copy for Value<R> {}

impl<R: RegisterValue> Default for Value<R> {
    #[inline]
    fn default() -> Value<R> {
        Value {
            value: R::RESET,
            _reg: PhantomData,
        }
    }
}

impl<R: RegisterValue> Debug for Value<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if fmt.alternate() {
            write!(
                fmt,
                "Value<{}>(0b{:02$b})",
                R::NAME,
                self.value,
                <R::Int as Int>::WIDTH
            )
        } else {
            write!(
                fmt,
                "Value<{}>(0x{:02$x})",
                R::NAME,
                self.value,
                <R::Int as Int>::WIDTH / 4
            )
        }
    }
}

impl<R: RegisterValue, T: Into<Value<R>> + Copy> PartialEq<T> for Value<R> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let other = (*other).into();
        self.value == other.value
    }
}

impl<R: RegisterValue> Eq for Value<R> {}

impl<R: RegisterValue, T: Into<FieldValues<R>>> BitOr<T> for Value<R> {
    type Output = Value<R>;

    #[inline]
    fn bitor(self, other: T) -> Self::Output {
        let other = other.into();
        Value {
            value: self.value & !other.mask() | other.bits(),
            _reg: PhantomData,
        }
    }
}

impl<R: RegisterValue, T: Into<FieldValues<R>>> BitOrAssign<T> for Value<R> {
    #[inline]
    fn bitor_assign(&mut self, other: T) {
        let other = other.into();
        self.value = self.value & !other.mask() | other.bits();
    }
}

impl<R: RegisterValue, T: Into<Fields<R>> + MayToggle> BitAnd<T> for Value<R> {
    type Output = FieldValues<R, T::Toggle>;

    #[inline]
    fn bitand(self, other: T) -> Self::Output {
        let other = other.into();
        unsafe { FieldValues::from_raw(self.value & other.mask(), other.mask()) }
    }
}

impl<R: RegisterValue, T: Into<Fields<R, Toggle>>> BitXor<T> for Value<R> {
    type Output = Value<R>;

    #[inline]
    fn bitxor(self, other: T) -> Self::Output {
        let other = other.into();
        Value {
            value: self.value ^ other.mask(),
            _reg: PhantomData,
        }
    }
}

impl<R: RegisterValue, T: Into<Fields<R, Toggle>>> BitXorAssign<T> for Value<R> {
    #[inline]
    fn bitxor_assign(&mut self, other: T) {
        let other = other.into();
        self.value = self.value ^ other.mask();
    }
}
