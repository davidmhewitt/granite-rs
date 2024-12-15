// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, SettingsColorScheme};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GraniteSettings")]
    pub struct Settings(Object<ffi::GraniteSettings, ffi::GraniteSettingsClass>);

    match fn {
        type_ => || ffi::granite_settings_get_type(),
    }
}

impl Settings {
    pub const NONE: Option<&'static Settings> = None;

    #[doc(alias = "granite_settings_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Settings {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::granite_settings_get_default()) }
    }
}

pub trait SettingsExt: IsA<Settings> + 'static {
    #[doc(alias = "granite_settings_get_prefers_color_scheme")]
    #[doc(alias = "get_prefers_color_scheme")]
    fn prefers_color_scheme(&self) -> SettingsColorScheme {
        unsafe {
            from_glib(ffi::granite_settings_get_prefers_color_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "prefers-color-scheme")]
    fn set_prefers_color_scheme(&self, prefers_color_scheme: SettingsColorScheme) {
        ObjectExt::set_property(self.as_ref(), "prefers-color-scheme", prefers_color_scheme)
    }

    #[doc(alias = "prefers-color-scheme")]
    fn connect_prefers_color_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_prefers_color_scheme_trampoline<
            P: IsA<Settings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Settings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::prefers-color-scheme\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_prefers_color_scheme_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Settings>> SettingsExt for O {}
