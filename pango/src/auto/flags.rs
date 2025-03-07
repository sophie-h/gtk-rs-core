// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "PangoFontMask")]
    pub struct FontMask: u32 {
        #[doc(alias = "PANGO_FONT_MASK_FAMILY")]
        const FAMILY = ffi::PANGO_FONT_MASK_FAMILY as u32;
        #[doc(alias = "PANGO_FONT_MASK_STYLE")]
        const STYLE = ffi::PANGO_FONT_MASK_STYLE as u32;
        #[doc(alias = "PANGO_FONT_MASK_VARIANT")]
        const VARIANT = ffi::PANGO_FONT_MASK_VARIANT as u32;
        #[doc(alias = "PANGO_FONT_MASK_WEIGHT")]
        const WEIGHT = ffi::PANGO_FONT_MASK_WEIGHT as u32;
        #[doc(alias = "PANGO_FONT_MASK_STRETCH")]
        const STRETCH = ffi::PANGO_FONT_MASK_STRETCH as u32;
        #[doc(alias = "PANGO_FONT_MASK_SIZE")]
        const SIZE = ffi::PANGO_FONT_MASK_SIZE as u32;
        #[doc(alias = "PANGO_FONT_MASK_GRAVITY")]
        const GRAVITY = ffi::PANGO_FONT_MASK_GRAVITY as u32;
        #[doc(alias = "PANGO_FONT_MASK_VARIATIONS")]
        const VARIATIONS = ffi::PANGO_FONT_MASK_VARIATIONS as u32;
    }
}

impl fmt::Display for FontMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FontMask {
    type GlibType = ffi::PangoFontMask;

    fn into_glib(self) -> ffi::PangoFontMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoFontMask> for FontMask {
    unsafe fn from_glib(value: ffi::PangoFontMask) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FontMask {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_font_mask_get_type()) }
    }
}

impl glib::value::ValueType for FontMask {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FontMask {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FontMask {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
bitflags! {
    #[doc(alias = "PangoShapeFlags")]
    pub struct ShapeFlags: u32 {
        #[doc(alias = "PANGO_SHAPE_NONE")]
        const NONE = ffi::PANGO_SHAPE_NONE as u32;
        #[doc(alias = "PANGO_SHAPE_ROUND_POSITIONS")]
        const ROUND_POSITIONS = ffi::PANGO_SHAPE_ROUND_POSITIONS as u32;
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShapeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl IntoGlib for ShapeFlags {
    type GlibType = ffi::PangoShapeFlags;

    fn into_glib(self) -> ffi::PangoShapeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShapeFlags> for ShapeFlags {
    unsafe fn from_glib(value: ffi::PangoShapeFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl StaticType for ShapeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_shape_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShapeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShapeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl ToValue for ShapeFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
bitflags! {
    #[doc(alias = "PangoShowFlags")]
    pub struct ShowFlags: u32 {
        #[doc(alias = "PANGO_SHOW_NONE")]
        const NONE = ffi::PANGO_SHOW_NONE as u32;
        #[doc(alias = "PANGO_SHOW_SPACES")]
        const SPACES = ffi::PANGO_SHOW_SPACES as u32;
        #[doc(alias = "PANGO_SHOW_LINE_BREAKS")]
        const LINE_BREAKS = ffi::PANGO_SHOW_LINE_BREAKS as u32;
        #[doc(alias = "PANGO_SHOW_IGNORABLES")]
        const IGNORABLES = ffi::PANGO_SHOW_IGNORABLES as u32;
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShowFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl IntoGlib for ShowFlags {
    type GlibType = ffi::PangoShowFlags;

    fn into_glib(self) -> ffi::PangoShowFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShowFlags> for ShowFlags {
    unsafe fn from_glib(value: ffi::PangoShowFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl StaticType for ShowFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_show_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShowFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShowFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl ToValue for ShowFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
