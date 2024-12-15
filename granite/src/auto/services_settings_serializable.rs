// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GraniteServicesSettingsSerializable")]
    pub struct ServicesSettingsSerializable(Interface<ffi::GraniteServicesSettingsSerializable, ffi::GraniteServicesSettingsSerializableIface>);

    match fn {
        type_ => || ffi::granite_services_settings_serializable_get_type(),
    }
}

impl ServicesSettingsSerializable {
    pub const NONE: Option<&'static ServicesSettingsSerializable> = None;
}

pub trait ServicesSettingsSerializableExt: IsA<ServicesSettingsSerializable> + 'static {
    #[doc(alias = "granite_services_settings_serializable_settings_serialize")]
    fn settings_serialize(&self) -> glib::GString {
        unsafe {
            from_glib_full(
                ffi::granite_services_settings_serializable_settings_serialize(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "granite_services_settings_serializable_settings_deserialize")]
    fn settings_deserialize(&self, s: &str) {
        unsafe {
            ffi::granite_services_settings_serializable_settings_deserialize(
                self.as_ref().to_glib_none().0,
                s.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<ServicesSettingsSerializable>> ServicesSettingsSerializableExt for O {}
