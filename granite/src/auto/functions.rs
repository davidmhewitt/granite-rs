// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

#[doc(alias = "granite_date_time_get_default_time_format")]
pub fn date_time_get_default_time_format(is_12h: bool, with_second: bool) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::granite_date_time_get_default_time_format(
            is_12h.into_glib(),
            with_second.into_glib(),
        ))
    }
}

#[doc(alias = "granite_date_time_get_relative_datetime")]
pub fn date_time_get_relative_datetime(date_time: &glib::DateTime) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::granite_date_time_get_relative_datetime(
            date_time.to_glib_none().0,
        ))
    }
}

#[doc(alias = "granite_date_time_is_same_day")]
pub fn date_time_is_same_day(day1: &glib::DateTime, day2: &glib::DateTime) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::granite_date_time_is_same_day(
            day1.to_glib_none().0,
            day2.to_glib_none().0,
        ))
    }
}

#[doc(alias = "granite_date_time_get_default_date_format")]
pub fn date_time_get_default_date_format(
    with_weekday: bool,
    with_day: bool,
    with_year: bool,
) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::granite_date_time_get_default_date_format(
            with_weekday.into_glib(),
            with_day.into_glib(),
            with_year.into_glib(),
        ))
    }
}

#[doc(alias = "granite_date_time_seconds_to_time")]
pub fn date_time_seconds_to_time(seconds: i32) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::granite_date_time_seconds_to_time(seconds)) }
}

//#[doc(alias = "granite_services_application_set_badge")]
//pub fn services_application_set_badge(count: i64, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:granite_services_application_set_badge() }
//}

//#[doc(alias = "granite_services_application_set_badge_visible")]
//pub fn services_application_set_badge_visible(visible: bool, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:granite_services_application_set_badge_visible() }
//}

//#[doc(alias = "granite_services_application_set_progress")]
//pub fn services_application_set_progress(progress: f64, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:granite_services_application_set_progress() }
//}

//#[doc(alias = "granite_services_application_set_progress_visible")]
//pub fn services_application_set_progress_visible(visible: bool, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:granite_services_application_set_progress_visible() }
//}

#[doc(alias = "granite_widgets_utils_set_color_primary")]
pub fn widgets_utils_set_color_primary(
    window: &impl IsA<gtk::Widget>,
    color: &mut gdk::RGBA,
    priority: i32,
) -> Option<gtk::CssProvider> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::granite_widgets_utils_set_color_primary(
            window.as_ref().to_glib_none().0,
            color.to_glib_none_mut().0,
            priority,
        ))
    }
}

#[cfg(feature = "v7_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v7_2")))]
#[doc(alias = "granite_init")]
pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::granite_init();
    }
}

#[doc(alias = "granite_accel_to_string")]
pub fn accel_to_string(accel: Option<&str>) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::granite_accel_to_string(accel.to_glib_none().0)) }
}

#[doc(alias = "granite_markup_accel_tooltip")]
pub fn markup_accel_tooltip(accels: &[&str], description: Option<&str>) -> glib::GString {
    assert_initialized_main_thread!();
    let accels_length1 = accels.len() as _;
    unsafe {
        from_glib_full(ffi::granite_markup_accel_tooltip(
            accels.to_glib_none().0,
            accels_length1,
            description.to_glib_none().0,
        ))
    }
}

#[doc(alias = "granite_contrasting_foreground_color")]
pub fn contrasting_foreground_color(bg_color: &mut gdk::RGBA) -> gdk::RGBA {
    assert_initialized_main_thread!();
    unsafe {
        let mut result = gdk::RGBA::uninitialized();
        ffi::granite_contrasting_foreground_color(
            bg_color.to_glib_none_mut().0,
            result.to_glib_none_mut().0,
        );
        result
    }
}
