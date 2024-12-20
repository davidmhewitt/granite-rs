// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GraniteModeSwitch")]
    pub struct ModeSwitch(Object<ffi::GraniteModeSwitch, ffi::GraniteModeSwitchClass>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::granite_mode_switch_get_type(),
    }
}

impl ModeSwitch {
    pub const NONE: Option<&'static ModeSwitch> = None;

    #[doc(alias = "granite_mode_switch_new")]
    pub fn new(
        primary_icon_gicon: &impl IsA<gio::Icon>,
        secondary_icon_gicon: &impl IsA<gio::Icon>,
    ) -> ModeSwitch {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::granite_mode_switch_new(
                primary_icon_gicon.as_ref().to_glib_none().0,
                secondary_icon_gicon.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_new_from_icon_name")]
    pub fn from_icon_name(primary_icon_name: &str, secondary_icon_name: &str) -> ModeSwitch {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::granite_mode_switch_new_from_icon_name(
                primary_icon_name.to_glib_none().0,
                secondary_icon_name.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ModeSwitch`] objects.
    ///
    /// This method returns an instance of [`ModeSwitchBuilder`](crate::builders::ModeSwitchBuilder) which can be used to create [`ModeSwitch`] objects.
    pub fn builder() -> ModeSwitchBuilder {
        ModeSwitchBuilder::new()
    }
}

impl Default for ModeSwitch {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ModeSwitch`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ModeSwitchBuilder {
    builder: glib::object::ObjectBuilder<'static, ModeSwitch>,
}

impl ModeSwitchBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn primary_icon_gicon(self, primary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-gicon", primary_icon_gicon.clone().upcast()),
        }
    }

    pub fn primary_icon_name(self, primary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-name", primary_icon_name.into()),
        }
    }

    pub fn primary_icon_tooltip_text(
        self,
        primary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-tooltip-text",
                primary_icon_tooltip_text.into(),
            ),
        }
    }

    pub fn secondary_icon_gicon(self, secondary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-gicon",
                secondary_icon_gicon.clone().upcast(),
            ),
        }
    }

    pub fn secondary_icon_name(self, secondary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-name", secondary_icon_name.into()),
        }
    }

    pub fn secondary_icon_tooltip_text(
        self,
        secondary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-tooltip-text",
                secondary_icon_tooltip_text.into(),
            ),
        }
    }

    #[cfg(feature = "gtk_v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_12")))]
    pub fn baseline_child(self, baseline_child: i32) -> Self {
        Self {
            builder: self.builder.property("baseline-child", baseline_child),
        }
    }

    //pub fn baseline_position(self, baseline_position: /*Ignored*/gtk::BaselinePosition) -> Self {
    //    Self { builder: self.builder.property("baseline-position", baseline_position), }
    //}

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    //pub fn layout_manager(self, layout_manager: &impl IsA</*Ignored*/gtk::LayoutManager>) -> Self {
    //    Self { builder: self.builder.property("layout-manager", layout_manager.clone().upcast()), }
    //}

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    //pub fn accessible_role(self, accessible_role: /*Ignored*/gtk::AccessibleRole) -> Self {
    //    Self { builder: self.builder.property("accessible-role", accessible_role), }
    //}

    //pub fn orientation(self, orientation: /*Ignored*/gtk::Orientation) -> Self {
    //    Self { builder: self.builder.property("orientation", orientation), }
    //}

    // rustdoc-stripper-ignore-next
    /// Build the [`ModeSwitch`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ModeSwitch {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait ModeSwitchExt: IsA<ModeSwitch> + 'static {
    #[doc(alias = "granite_mode_switch_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::granite_mode_switch_get_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_active")]
    fn set_active(&self, value: bool) {
        unsafe {
            ffi::granite_mode_switch_set_active(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "granite_mode_switch_get_primary_icon_gicon")]
    #[doc(alias = "get_primary_icon_gicon")]
    fn primary_icon_gicon(&self) -> gio::Icon {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_primary_icon_gicon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_primary_icon_gicon")]
    fn set_primary_icon_gicon(&self, value: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::granite_mode_switch_set_primary_icon_gicon(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_mode_switch_get_primary_icon_name")]
    #[doc(alias = "get_primary_icon_name")]
    fn primary_icon_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_primary_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_primary_icon_name")]
    fn set_primary_icon_name(&self, value: &str) {
        unsafe {
            ffi::granite_mode_switch_set_primary_icon_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_mode_switch_get_primary_icon_tooltip_text")]
    #[doc(alias = "get_primary_icon_tooltip_text")]
    fn primary_icon_tooltip_text(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_primary_icon_tooltip_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_primary_icon_tooltip_text")]
    fn set_primary_icon_tooltip_text(&self, value: &str) {
        unsafe {
            ffi::granite_mode_switch_set_primary_icon_tooltip_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_mode_switch_get_secondary_icon_gicon")]
    #[doc(alias = "get_secondary_icon_gicon")]
    fn secondary_icon_gicon(&self) -> gio::Icon {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_secondary_icon_gicon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_secondary_icon_gicon")]
    fn set_secondary_icon_gicon(&self, value: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::granite_mode_switch_set_secondary_icon_gicon(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_mode_switch_get_secondary_icon_name")]
    #[doc(alias = "get_secondary_icon_name")]
    fn secondary_icon_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_secondary_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_secondary_icon_name")]
    fn set_secondary_icon_name(&self, value: &str) {
        unsafe {
            ffi::granite_mode_switch_set_secondary_icon_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_mode_switch_get_secondary_icon_tooltip_text")]
    #[doc(alias = "get_secondary_icon_tooltip_text")]
    fn secondary_icon_tooltip_text(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_mode_switch_get_secondary_icon_tooltip_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_mode_switch_set_secondary_icon_tooltip_text")]
    fn set_secondary_icon_tooltip_text(&self, value: &str) {
        unsafe {
            ffi::granite_mode_switch_set_secondary_icon_tooltip_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<ModeSwitch>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "primary-icon-gicon")]
    fn connect_primary_icon_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_icon_gicon_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::primary-icon-gicon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_primary_icon_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "primary-icon-name")]
    fn connect_primary_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_icon_name_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::primary-icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_primary_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "primary-icon-tooltip-text")]
    fn connect_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_icon_tooltip_text_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::primary-icon-tooltip-text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_primary_icon_tooltip_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "secondary-icon-gicon")]
    fn connect_secondary_icon_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_icon_gicon_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-icon-gicon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_secondary_icon_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "secondary-icon-name")]
    fn connect_secondary_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_icon_name_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_secondary_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "secondary-icon-tooltip-text")]
    fn connect_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_icon_tooltip_text_trampoline<
            P: IsA<ModeSwitch>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteModeSwitch,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ModeSwitch::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-icon-tooltip-text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_secondary_icon_tooltip_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ModeSwitch>> ModeSwitchExt for O {}
