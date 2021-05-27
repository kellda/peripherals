use super::*;

/// A register from a peripheral only known at runtime
///
/// This allows to choose at runtime which instance of a peripheral to use. It has the same methods
/// than the [`Reg`] type. It is created by [`Reg::into_dyn`] or as part of the struct created by
/// `peripheral.into_dyn()`.
pub struct DynReg<R> {
    _reg: PhantomData<R>,
}

impl<R: Register> DynReg<R> {
    /// Raw pointer to the register
    #[inline]
    pub fn ptr(&self) -> *const R::Int {
        (self as *const _ as usize + R::OFFSET) as *const R::Int
    }

    /// Mutable raw pointer to the register
    #[inline]
    pub fn ptr_mut(&mut self) -> *mut R::Int {
        (self as *mut _ as usize + R::OFFSET) as *mut R::Int
    }
}

impl<R: ReadRegister> DynReg<R> {
    /// Read the current value of this register
    ///
    /// This returns a [`Value`], which can be used to read fields or modified and written back.
    #[inline]
    pub fn read(&self) -> Value<R::Value> {
        unsafe { Value::from_raw(self.ptr().read_volatile()) }
    }

    /// Read the given field
    ///
    /// Same as `register.read().field(fields)`. See [`Value::field`] for more details.
    #[inline]
    pub fn field<T>(&self, field: Field<R::Value, T, R::Int>) -> T
    where
        R::Int: TryInto<T>,
        <R::Int as TryInto<T>>::Error: Debug,
    {
        self.read().field(field)
    }

    /// Read the given fields
    ///
    /// Same as `register.read() & fields`. See [`Value`] for more details.
    #[inline]
    pub fn fields<F: Into<Fields<R::Value>> + MayToggle>(
        &self,
        fields: F,
    ) -> FieldValues<R::Value, F::Toggle> {
        self.read() & fields
    }

    /// Test the value of the given fields
    ///
    /// Same as `register.read().test(bits)`. See [`Value::test`] for more details.
    #[inline]
    pub fn test<B: Into<FieldValues<R::Value>>>(&self, bits: B) -> bool {
        self.read().test(bits)
    }
}

impl<R: WriteRegister> DynReg<R> {
    /// Write a value to this register
    ///
    /// This takes a [`Value`], which can be read from a register or created with `Default`.
    #[inline]
    pub fn write(&mut self, value: Value<R::Value>) {
        unsafe {
            self.ptr_mut().write_volatile(value.value());
        }
    }

    /// Reset this register
    ///
    /// This set the register to the value it has right after a reset or a boot.
    #[inline]
    pub fn reset(&mut self) {
        self.write(Value::reset());
    }
}

impl<R: ReadRegister + WriteRegister> DynReg<R> {
    /// Modify the given fields
    ///
    /// This takes any field defined for this register by the [`periph!`] macro. These fields can
    /// also be combined with the `|` operator.
    #[inline]
    pub fn modify<B: Into<FieldValues<R::Value>>>(&mut self, bits: B) {
        self.write(self.read() | bits);
    }

    /// Toggle the given fields
    ///
    /// This takes a toggleable field defined for this register by the [`periph!`] macro. These
    /// fields can also be combined with the `|` operator.
    #[inline]
    pub fn toggle<F: Into<Fields<R::Value, Toggle>>>(&mut self, fields: F) {
        let fields = fields.into();
        self.write(self.read() ^ fields);
    }
}

impl<R: Register> Debug for DynReg<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "DynReg {} @ 0x{:06p}", R::NAME, self.ptr())
    }
}
