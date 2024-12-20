// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GraniteServicesContract")]
    pub struct ServicesContract(Interface<ffi::GraniteServicesContract, ffi::GraniteServicesContractIface>);

    match fn {
        type_ => || ffi::granite_services_contract_get_type(),
    }
}

impl ServicesContract {
    pub const NONE: Option<&'static ServicesContract> = None;
}

pub trait ServicesContractExt: IsA<ServicesContract> + 'static {
    #[doc(alias = "granite_services_contract_get_display_name")]
    #[doc(alias = "get_display_name")]
    fn display_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::granite_services_contract_get_display_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_services_contract_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::granite_services_contract_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_services_contract_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> gio::Icon {
        unsafe {
            from_glib_full(ffi::granite_services_contract_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_services_contract_execute_with_file")]
    fn execute_with_file(&self, file: &impl IsA<gio::File>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::granite_services_contract_execute_with_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "granite_services_contract_execute_with_files")]
    fn execute_with_files(&self, files: &[gio::File]) -> Result<(), glib::Error> {
        let files_length1 = files.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::granite_services_contract_execute_with_files(
                self.as_ref().to_glib_none().0,
                files.to_glib_none().0,
                files_length1,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<ServicesContract>> ServicesContractExt for O {}
