use uuid::Uuid;
use wasm_bindgen::JsValue;

//! This file is heavily inspired by the rusqlite library. And provides a mostly API compatible
//! interface for use with WebAssembly
//!
//! https://github.com/rusqlite/rusqlite/blob/master/src/lib.rs

pub trait ToSql {
    fn to_sql(&self) -> JsValue;
}
impl ToSql for u8 {
    fn to_sql(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}
impl ToSql for String {
    fn to_sql(&self) -> JsValue {
        JsValue::from_str(self)
    }
}
impl ToSql for &str {
    fn to_sql(&self) -> JsValue {
        JsValue::from_str(self)
    }
}
impl ToSql for Uuid {
    fn to_sql(&self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

pub trait FromSql {
    fn from_sql(value: JsValue) -> Self;
}
impl FromSql for u8 {
    fn from_sql(value: JsValue) -> Self {
        value.as_f64().unwrap() as u8
    }
}
impl FromSql for i64 {
    fn from_sql(value: JsValue) -> Self {
        value.as_f64().unwrap() as i64
    }
}
impl FromSql for String {
    fn from_sql(value: JsValue) -> Self {
        value.as_string().unwrap()
    }
}
impl FromSql for Uuid {
    fn from_sql(value: JsValue) -> Self {
        Uuid::parse_str(&value.as_string().unwrap()).unwrap()
    }
}

pub trait Params {
    fn to_sql(&self) -> JsValue;
}
impl Params for [&(dyn ToSql + Send + Sync); 0] {
    fn to_sql(&self) -> JsValue {
        JsValue::NULL
    }
}
impl Params for &[&dyn ToSql] {
    fn to_sql(&self) -> JsValue {
        let array = js_sys::Array::new();
        for item in *self {
            array.push(&item.to_sql());
        }
        array.into()
    }
}
impl Params for &[(&str, &dyn ToSql)] {
    fn to_sql(&self) -> JsValue {
        let object = js_sys::Object::new();
        for (key, value) in *self {
            js_sys::Reflect::set(&object, &JsValue::from_str(key), &value.to_sql()).unwrap();
        }
        object.into()
    }
}

#[macro_export]
macro_rules! params {
    () => {
        &[] as &[&dyn $crate::ToSql]
    };
    ($($param:expr),+ $(,)?) => {
        &[$(&$param as &dyn $crate::ToSql),+] as &[&dyn $crate::ToSql]
    };
}

#[macro_export]
macro_rules! named_params {
    () => {
        &[] as &[(&str, &dyn $crate::ToSql)]
    };
    // Note: It's a lot more work to support this as part of the same macro as
    // `params!`, unfortunately.
    ($($param_name:literal: $param_val:expr),+ $(,)?) => {
        &[$(($param_name, &$param_val as &dyn $crate::ToSql)),+] as &[(&str, &dyn $crate::ToSql)]
    };
}
