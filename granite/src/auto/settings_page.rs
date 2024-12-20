// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, SettingsPageStatusType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GraniteSettingsPage")]
    pub struct SettingsPage(Object<ffi::GraniteSettingsPage, ffi::GraniteSettingsPageClass>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::granite_settings_page_get_type(),
    }
}

impl SettingsPage {
    pub const NONE: Option<&'static SettingsPage> = None;
}

pub trait SettingsPageExt: IsA<SettingsPage> + 'static {
    #[doc(alias = "granite_settings_page_get_status_type")]
    #[doc(alias = "get_status_type")]
    fn status_type(&self) -> SettingsPageStatusType {
        unsafe {
            from_glib(ffi::granite_settings_page_get_status_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_set_status_type")]
    fn set_status_type(&self, value: SettingsPageStatusType) {
        unsafe {
            ffi::granite_settings_page_set_status_type(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "granite_settings_page_get_display_widget")]
    #[doc(alias = "get_display_widget")]
    fn display_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_display_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_get_header")]
    #[doc(alias = "get_header")]
    fn header(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_get_status")]
    #[doc(alias = "get_status")]
    fn status(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_status(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_set_status")]
    fn set_status(&self, value: &str) {
        unsafe {
            ffi::granite_settings_page_set_status(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_settings_page_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_set_icon_name")]
    fn set_icon_name(&self, value: Option<&str>) {
        unsafe {
            ffi::granite_settings_page_set_icon_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_settings_page_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_set_title")]
    fn set_title(&self, value: &str) {
        unsafe {
            ffi::granite_settings_page_set_title(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_settings_page_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> gtk::Widget {
        unsafe {
            from_glib_none(ffi::granite_settings_page_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_settings_page_set_child")]
    fn set_child(&self, value: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::granite_settings_page_set_child(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "status-type")]
    fn connect_status_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_type_trampoline<
            P: IsA<SettingsPage>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteSettingsPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SettingsPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::status-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_status_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "status")]
    fn connect_status_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_trampoline<P: IsA<SettingsPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteSettingsPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SettingsPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::status\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_status_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<
            P: IsA<SettingsPage>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteSettingsPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SettingsPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<SettingsPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteSettingsPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SettingsPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<SettingsPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteSettingsPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SettingsPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SettingsPage>> SettingsPageExt for O {}
