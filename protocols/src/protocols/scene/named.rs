pub trait Named {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);
}

macro_rules! impl_named {
    ($object:ident) => {
        impl Named for $object {
            fn name(&self) -> &String { &self.name }

            fn set_name(&mut self, name: String) {
                self.name = name;
            }
        }
    }
}