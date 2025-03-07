// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GCharsetConverter")]
    pub struct CharsetConverter(Object<ffi::GCharsetConverter, ffi::GCharsetConverterClass>) @implements Converter;

    match fn {
        type_ => || ffi::g_charset_converter_get_type(),
    }
}

impl CharsetConverter {
    #[doc(alias = "g_charset_converter_new")]
    pub fn new(to_charset: &str, from_charset: &str) -> Result<CharsetConverter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_charset_converter_new(
                to_charset.to_glib_none().0,
                from_charset.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`CharsetConverter`].
    ///
    /// This method returns an instance of [`CharsetConverterBuilder`] which can be used to create a [`CharsetConverter`].
    pub fn builder() -> CharsetConverterBuilder {
        CharsetConverterBuilder::default()
    }

    #[doc(alias = "g_charset_converter_get_num_fallbacks")]
    #[doc(alias = "get_num_fallbacks")]
    pub fn num_fallbacks(&self) -> u32 {
        unsafe { ffi::g_charset_converter_get_num_fallbacks(self.to_glib_none().0) }
    }

    #[doc(alias = "g_charset_converter_get_use_fallback")]
    #[doc(alias = "get_use_fallback")]
    pub fn uses_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_charset_converter_get_use_fallback(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_charset_converter_set_use_fallback")]
    pub fn set_use_fallback(&self, use_fallback: bool) {
        unsafe {
            ffi::g_charset_converter_set_use_fallback(
                self.to_glib_none().0,
                use_fallback.into_glib(),
            );
        }
    }

    #[doc(alias = "from-charset")]
    pub fn from_charset(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"from-charset\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `from-charset` getter")
        }
    }

    #[doc(alias = "to-charset")]
    pub fn to_charset(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"to-charset\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `to-charset` getter")
        }
    }

    #[doc(alias = "use-fallback")]
    pub fn connect_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_fallback_trampoline<F: Fn(&CharsetConverter) + 'static>(
            this: *mut ffi::GCharsetConverter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-fallback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_fallback_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`CharsetConverter`].
pub struct CharsetConverterBuilder {
    from_charset: Option<String>,
    to_charset: Option<String>,
    use_fallback: Option<bool>,
}

impl CharsetConverterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CharsetConverterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CharsetConverter`].
    pub fn build(self) -> CharsetConverter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref from_charset) = self.from_charset {
            properties.push(("from-charset", from_charset));
        }
        if let Some(ref to_charset) = self.to_charset {
            properties.push(("to-charset", to_charset));
        }
        if let Some(ref use_fallback) = self.use_fallback {
            properties.push(("use-fallback", use_fallback));
        }
        glib::Object::new::<CharsetConverter>(&properties)
            .expect("Failed to create an instance of CharsetConverter")
    }

    pub fn from_charset(mut self, from_charset: &str) -> Self {
        self.from_charset = Some(from_charset.to_string());
        self
    }

    pub fn to_charset(mut self, to_charset: &str) -> Self {
        self.to_charset = Some(to_charset.to_string());
        self
    }

    pub fn use_fallback(mut self, use_fallback: bool) -> Self {
        self.use_fallback = Some(use_fallback);
        self
    }
}

impl fmt::Display for CharsetConverter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CharsetConverter")
    }
}
