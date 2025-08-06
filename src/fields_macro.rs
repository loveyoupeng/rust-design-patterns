#[macro_export]
macro_rules! i32_getter {
    ( $name:ident ) => {
        i32::from_le_bytes($name)
    };
}
