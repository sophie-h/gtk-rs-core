// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FilterOutputStream;
use crate::OutputStream;
use crate::Seekable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GBufferedOutputStream")]
    pub struct BufferedOutputStream(Object<ffi::GBufferedOutputStream, ffi::GBufferedOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_buffered_output_stream_get_type(),
    }
}

impl BufferedOutputStream {
    #[doc(alias = "g_buffered_output_stream_new")]
    pub fn new<P: IsA<OutputStream>>(base_stream: &P) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "g_buffered_output_stream_new_sized")]
    pub fn new_sized<P: IsA<OutputStream>>(base_stream: &P, size: usize) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new_sized(
                base_stream.as_ref().to_glib_none().0,
                size,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`BufferedOutputStream`].
    ///
    /// This method returns an instance of [`BufferedOutputStreamBuilder`] which can be used to create a [`BufferedOutputStream`].
    pub fn builder() -> BufferedOutputStreamBuilder {
        BufferedOutputStreamBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`BufferedOutputStream`].
pub struct BufferedOutputStreamBuilder {
    auto_grow: Option<bool>,
    buffer_size: Option<u32>,
    base_stream: Option<OutputStream>,
    close_base_stream: Option<bool>,
}

impl BufferedOutputStreamBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`BufferedOutputStreamBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BufferedOutputStream`].
    pub fn build(self) -> BufferedOutputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref auto_grow) = self.auto_grow {
            properties.push(("auto-grow", auto_grow));
        }
        if let Some(ref buffer_size) = self.buffer_size {
            properties.push(("buffer-size", buffer_size));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new::<BufferedOutputStream>(&properties)
            .expect("Failed to create an instance of BufferedOutputStream")
    }

    pub fn auto_grow(mut self, auto_grow: bool) -> Self {
        self.auto_grow = Some(auto_grow);
        self
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn base_stream<P: IsA<OutputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_BUFFERED_OUTPUT_STREAM: Option<&BufferedOutputStream> = None;

pub trait BufferedOutputStreamExt: 'static {
    #[doc(alias = "g_buffered_output_stream_get_auto_grow")]
    #[doc(alias = "get_auto_grow")]
    fn auto_grows(&self) -> bool;

    #[doc(alias = "g_buffered_output_stream_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    fn buffer_size(&self) -> usize;

    #[doc(alias = "g_buffered_output_stream_set_auto_grow")]
    fn set_auto_grow(&self, auto_grow: bool);

    #[doc(alias = "g_buffered_output_stream_set_buffer_size")]
    fn set_buffer_size(&self, size: usize);

    #[doc(alias = "auto-grow")]
    fn connect_auto_grow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "buffer-size")]
    fn connect_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BufferedOutputStream>> BufferedOutputStreamExt for O {
    fn auto_grows(&self) -> bool {
        unsafe {
            from_glib(ffi::g_buffered_output_stream_get_auto_grow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn buffer_size(&self) -> usize {
        unsafe { ffi::g_buffered_output_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    fn set_auto_grow(&self, auto_grow: bool) {
        unsafe {
            ffi::g_buffered_output_stream_set_auto_grow(
                self.as_ref().to_glib_none().0,
                auto_grow.into_glib(),
            );
        }
    }

    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_output_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    #[doc(alias = "auto-grow")]
    fn connect_auto_grow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_grow_trampoline<
            P: IsA<BufferedOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GBufferedOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&BufferedOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-grow\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_grow_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffer-size")]
    fn connect_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<
            P: IsA<BufferedOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GBufferedOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&BufferedOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BufferedOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BufferedOutputStream")
    }
}
