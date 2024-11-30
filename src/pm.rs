use paste::paste;

macro_rules! pm_mods {
    ( $( $vis:vis $mod:ident; )+ ) => {
        $(
            $vis mod $mod;
            paste! { pub use self::$mod::[<$mod:camel>]; }
        )+
    }
}

pm_mods! {
    dnf;
}
