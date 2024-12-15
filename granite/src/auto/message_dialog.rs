// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Dialog};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GraniteMessageDialog")]
    pub struct MessageDialog(Object<ffi::GraniteMessageDialog, ffi::GraniteMessageDialogClass>) @extends Dialog, gtk::Dialog, gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        type_ => || ffi::granite_message_dialog_get_type(),
    }
}

impl MessageDialog {
    pub const NONE: Option<&'static MessageDialog> = None;

    #[doc(alias = "granite_message_dialog_new")]
    pub fn new(
        primary_text: &str,
        secondary_text: &str,
        image_icon: &impl IsA<gio::Icon>,
        buttons: gtk::ButtonsType,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::granite_message_dialog_new(
                primary_text.to_glib_none().0,
                secondary_text.to_glib_none().0,
                image_icon.as_ref().to_glib_none().0,
                buttons.into_glib(),
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_new_with_image_from_icon_name")]
    pub fn with_image_from_icon_name(
        primary_text: &str,
        secondary_text: &str,
        image_icon_name: &str,
        buttons: gtk::ButtonsType,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::granite_message_dialog_new_with_image_from_icon_name(
                primary_text.to_glib_none().0,
                secondary_text.to_glib_none().0,
                image_icon_name.to_glib_none().0,
                buttons.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MessageDialog`] objects.
    ///
    /// This method returns an instance of [`MessageDialogBuilder`](crate::builders::MessageDialogBuilder) which can be used to create [`MessageDialog`] objects.
    pub fn builder() -> MessageDialogBuilder {
        MessageDialogBuilder::new()
    }
}

impl Default for MessageDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MessageDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MessageDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, MessageDialog>,
}

impl MessageDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn primary_text(self, primary_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("primary-text", primary_text.into()),
        }
    }

    pub fn secondary_text(self, secondary_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-text", secondary_text.into()),
        }
    }

    pub fn image_icon(self, image_icon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("image-icon", image_icon.clone().upcast()),
        }
    }

    pub fn badge_icon(self, badge_icon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("badge-icon", badge_icon.clone().upcast()),
        }
    }

    pub fn primary_label(self, primary_label: &gtk::Label) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-label", primary_label.clone()),
        }
    }

    pub fn secondary_label(self, secondary_label: &gtk::Label) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-label", secondary_label.clone()),
        }
    }

    pub fn buttons(self, buttons: gtk::ButtonsType) -> Self {
        Self {
            builder: self.builder.property("buttons", buttons),
        }
    }

    pub fn custom_bin(self, custom_bin: &impl IsA<gtk::Box>) -> Self {
        Self {
            builder: self
                .builder
                .property("custom-bin", custom_bin.clone().upcast()),
        }
    }

    //pub fn application(self, application: &impl IsA</*Ignored*/gtk::Application>) -> Self {
    //    Self { builder: self.builder.property("application", application.clone().upcast()), }
    //}

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    //pub fn display(self, display: /*Ignored*/&gdk::Display) -> Self {
    //    Self { builder: self.builder.property("display", display), }
    //}

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget(self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "gtk_v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "gtk_v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<gtk::Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`MessageDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MessageDialog {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait MessageDialogExt: IsA<MessageDialog> + 'static {
    #[doc(alias = "granite_message_dialog_get_primary_text")]
    #[doc(alias = "get_primary_text")]
    fn primary_text(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_message_dialog_get_primary_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_set_primary_text")]
    fn set_primary_text(&self, value: &str) {
        unsafe {
            ffi::granite_message_dialog_set_primary_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_message_dialog_get_secondary_text")]
    #[doc(alias = "get_secondary_text")]
    fn secondary_text(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::granite_message_dialog_get_secondary_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_set_secondary_text")]
    fn set_secondary_text(&self, value: &str) {
        unsafe {
            ffi::granite_message_dialog_set_secondary_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_message_dialog_get_image_icon")]
    #[doc(alias = "get_image_icon")]
    fn image_icon(&self) -> gio::Icon {
        unsafe {
            from_glib_full(ffi::granite_message_dialog_get_image_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_set_image_icon")]
    fn set_image_icon(&self, value: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::granite_message_dialog_set_image_icon(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_message_dialog_get_badge_icon")]
    #[doc(alias = "get_badge_icon")]
    fn badge_icon(&self) -> gio::Icon {
        unsafe {
            from_glib_full(ffi::granite_message_dialog_get_badge_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_set_badge_icon")]
    fn set_badge_icon(&self, value: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::granite_message_dialog_set_badge_icon(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "granite_message_dialog_get_primary_label")]
    #[doc(alias = "get_primary_label")]
    fn primary_label(&self) -> gtk::Label {
        unsafe {
            from_glib_none(ffi::granite_message_dialog_get_primary_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_get_secondary_label")]
    #[doc(alias = "get_secondary_label")]
    fn secondary_label(&self) -> gtk::Label {
        unsafe {
            from_glib_none(ffi::granite_message_dialog_get_secondary_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_get_custom_bin")]
    #[doc(alias = "get_custom_bin")]
    fn custom_bin(&self) -> gtk::Box {
        unsafe {
            from_glib_none(ffi::granite_message_dialog_get_custom_bin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "granite_message_dialog_show_error_details")]
    fn show_error_details(&self, error_message: &str) {
        unsafe {
            ffi::granite_message_dialog_show_error_details(
                self.as_ref().to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "primary-text")]
    fn connect_primary_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_text_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::primary-text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_primary_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "secondary-text")]
    fn connect_secondary_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_text_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_secondary_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "image-icon")]
    fn connect_image_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_image_icon_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::image-icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_image_icon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "badge-icon")]
    fn connect_badge_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_badge_icon_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GraniteMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::badge-icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_badge_icon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<MessageDialog>> MessageDialogExt for O {}
