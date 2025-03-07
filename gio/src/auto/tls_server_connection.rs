// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IOStream;
use crate::TlsAuthenticationMode;
use crate::TlsCertificate;
use crate::TlsConnection;
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
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GTlsServerConnection")]
    pub struct TlsServerConnection(Interface<ffi::GTlsServerConnection, ffi::GTlsServerConnectionInterface>) @requires TlsConnection, IOStream;

    match fn {
        type_ => || ffi::g_tls_server_connection_get_type(),
    }
}

impl TlsServerConnection {
    #[doc(alias = "g_tls_server_connection_new")]
    pub fn new<P: IsA<IOStream>, Q: IsA<TlsCertificate>>(
        base_io_stream: &P,
        certificate: Option<&Q>,
    ) -> Result<TlsServerConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_server_connection_new(
                base_io_stream.as_ref().to_glib_none().0,
                certificate.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_TLS_SERVER_CONNECTION: Option<&TlsServerConnection> = None;

pub trait TlsServerConnectionExt: 'static {
    #[doc(alias = "authentication-mode")]
    fn authentication_mode(&self) -> TlsAuthenticationMode;

    #[doc(alias = "authentication-mode")]
    fn set_authentication_mode(&self, authentication_mode: TlsAuthenticationMode);

    #[doc(alias = "authentication-mode")]
    fn connect_authentication_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsServerConnection>> TlsServerConnectionExt for O {
    fn authentication_mode(&self) -> TlsAuthenticationMode {
        unsafe {
            let mut value =
                glib::Value::from_type(<TlsAuthenticationMode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"authentication-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `authentication-mode` getter")
        }
    }

    fn set_authentication_mode(&self, authentication_mode: TlsAuthenticationMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"authentication-mode\0".as_ptr() as *const _,
                authentication_mode.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "authentication-mode")]
    fn connect_authentication_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_authentication_mode_trampoline<
            P: IsA<TlsServerConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsServerConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsServerConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::authentication-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_authentication_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsServerConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsServerConnection")
    }
}
