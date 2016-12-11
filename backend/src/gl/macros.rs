macro_rules! impl_simple_globject {
    ($name:ident, $is:ident $(, $name_str:expr)*) => {
        impl GLObject for $name {
            #[inline(always)]
            fn raw(&self) -> GLuint { self.0 }

            #[inline(always)]
            fn into_raw(mut self) -> GLuint {
                mem::replace(&mut self.0, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                TRUE == unsafe { $is(self.0) }
            }

            #[inline(always)]
            fn check(&self) -> GLResult<()> {
                if self.is_valid() { Ok(()) } else {
                    $(error!("Invalid {}", $name_str);)*
                    Err(GLError::InvalidValue)
                }
            }
        }
    }
}