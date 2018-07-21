use std::iter::FromIterator;

use heck::CamelCase;
use syn;
use util;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypePosition {
    Argument,
    Return,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSize {
    I8 = 8,
    I16 = 16,
    I32 = 32,
    I64 = 64,
    // TODO: do we want "size" and "usize" options? Do they make sense?
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatSize {
    F32 = 32,
    F64 = 64,
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    Bool,
    Integral { signed: bool, size: IntSize },
    Floating(FloatSize),
    String,
    ByteString,
    // TODO: handle Result and Vec. (and slices?)
    JsValue,
    Interface(String),
    Dictionary(String),
    Enum(String),
}

impl TypeKind {
    pub fn is_primitive(&self) -> bool {
        match *self {
            TypeKind::Interface(_) => false,
            TypeKind::Dictionary(_) => false,
            TypeKind::Enum(_) => false,
            _ => true,
        }
    }

    pub fn to_syn_ty(&self, pos: TypePosition) -> Option<syn::Type> {
        Some(match self {
            // A reference to a type by name becomes the same thing in the
            // bindings.
            TypeKind::Interface(ref name) | TypeKind::Dictionary(ref name) | TypeKind::Enum(ref name) => {
                let ty = util::ident_ty(util::rust_ident(name.to_camel_case().as_str()));
                if let TypeKind::Interface(_) = self {
                    if pos == TypePosition::Argument {
                        shared_ref(ty)
                    } else {
                        ty
                    }
                } else {
                    ty
                }
            }

            // Scalars.
            TypeKind::Bool => util::ident_ty(util::raw_ident("bool")),
            TypeKind::Integral { signed: true, size: IntSize::I8 } => util::ident_ty(util::raw_ident("i8")),
            TypeKind::Integral { signed: false, size: IntSize::I8 } => util::ident_ty(util::raw_ident("u8")),
            TypeKind::Integral { signed: true, size: IntSize::I16 } => util::ident_ty(util::raw_ident("i16")),
            TypeKind::Integral { signed: false, size: IntSize::I16 } => util::ident_ty(util::raw_ident("u16")),
            TypeKind::Integral { signed: true, size: IntSize::I32 } => util::ident_ty(util::raw_ident("i32")),
            TypeKind::Integral { signed: false, size: IntSize::I32 } => util::ident_ty(util::raw_ident("u32")),
            TypeKind::Integral { signed: true, size: IntSize::I64 } => util::ident_ty(util::raw_ident("i64")),
            TypeKind::Integral { signed: false, size: IntSize::I64 } => util::ident_ty(util::raw_ident("u64")),
            TypeKind::Floating(FloatSize::F64) => util::ident_ty(util::raw_ident("f64")),
            TypeKind::Floating(FloatSize::F32) => util::ident_ty(util::raw_ident("f32")),

            TypeKind::String => match pos {
                TypePosition::Argument => shared_ref(util::ident_ty(util::raw_ident("str"))),
                _ => { return None; }, // `DOMString` is not supported yet in other positions.
            }

            TypeKind::ByteString => match pos {
                TypePosition::Argument => shared_ref(slice_ty(util::ident_ty(util::raw_ident("u8")))),
                TypePosition::Return => vec_ty(util::ident_ty(util::raw_ident("u8"))),
            }

            TypeKind::JsValue => util::simple_path_ty(vec![util::rust_ident("wasm_bindgen"), util::rust_ident("JsValue")]),
        })
    }
}

pub fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

fn slice_ty(t: syn::Type) -> syn::Type {
    syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(t),
    }.into()
}

fn vec_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
        ]),
        gt_token: Default::default(),
    });

    let ident = util::raw_ident("Vec");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}
