use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use bytes::Bytes;
use nutype::nutype;
use rhai::serde::from_dynamic;
use serde::{Deserialize, Serialize};
use serde_json::Number;

use reearth_flow_common::str::base64_encode;
use reearth_flow_common::uri::Uri;
use reearth_flow_common::xml::XmlXpathValue;

use crate::datetime::DateTime;
use crate::error;
use crate::error::Result;

#[nutype(
    sanitize(trim),
    derive(
        Debug,
        Display,
        Clone,
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
        AsRef,
        Serialize,
        Deserialize,
        Hash,
        JsonSchema
    )
)]
pub struct Attribute(String);

impl Attribute {
    pub fn inner(&self) -> String {
        self.clone().into_inner()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttributeValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    DateTime(DateTime),
    Array(Vec<AttributeValue>),
    Bytes(Bytes),
    Map(HashMap<String, AttributeValue>),
}

impl PartialEq for AttributeValue {
    fn eq(&self, rhs: &Self) -> bool {
        match (&self, &rhs) {
            (&AttributeValue::Null, &AttributeValue::Null) => true,
            (&AttributeValue::Bool(v0), &AttributeValue::Bool(v1)) if v0 == v1 => true,
            (&AttributeValue::Number(v0), &AttributeValue::Number(v1)) if v0 == v1 => true,
            (&AttributeValue::String(v0), &AttributeValue::String(v1)) if v0 == v1 => true,
            (&AttributeValue::DateTime(v0), &AttributeValue::DateTime(v1)) if v0 == v1 => true,
            (&AttributeValue::Array(v0), &AttributeValue::Array(v1)) if v0 == v1 => true,
            (&AttributeValue::Bytes(v0), &AttributeValue::Bytes(v1)) if v0 == v1 => true,
            (&AttributeValue::Map(v0), &AttributeValue::Map(v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl Ord for AttributeValue {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (&self, &rhs) {
            (&AttributeValue::Null, &AttributeValue::Null) => Ordering::Equal,
            (&AttributeValue::Bool(v0), &AttributeValue::Bool(v1)) => v0.cmp(v1),
            (&AttributeValue::Number(v0), &AttributeValue::Number(v1)) => {
                compare_numbers(v0, v1).unwrap()
            }
            (&AttributeValue::String(v0), &AttributeValue::String(v1)) => v0.cmp(v1),
            (&AttributeValue::DateTime(v0), &AttributeValue::DateTime(v1)) => v0.cmp(v1),
            (&AttributeValue::Array(v0), &AttributeValue::Array(v1)) => v0.cmp(v1),
            (&AttributeValue::Bytes(v0), &AttributeValue::Bytes(v1)) => v0.cmp(v1),
            (v0, v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl AttributeValue {
    fn discriminant(&self) -> usize {
        match *self {
            AttributeValue::Null => 0,
            AttributeValue::Bool(..) => 1,
            AttributeValue::Number(..) => 2,
            AttributeValue::String(..) => 3,
            AttributeValue::Array(..) => 4,
            AttributeValue::Bytes(..) => 5,
            AttributeValue::Map(..) => 6,
            AttributeValue::DateTime(..) => 7,
        }
    }

    pub fn extend(self, value: Self) -> Result<Self> {
        match (self, value) {
            (AttributeValue::Map(mut a), AttributeValue::Map(b)) => {
                for (k, v) in b {
                    a.insert(k, v);
                }
                Ok(AttributeValue::Map(a))
            }
            (AttributeValue::Array(mut a), AttributeValue::Array(b)) => {
                a.extend(b);
                Ok(AttributeValue::Array(a))
            }
            _ => Err(error::Error::internal_runtime("Cannot extend")),
        }
    }
}

impl Eq for AttributeValue {}
impl PartialOrd for AttributeValue {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Default for AttributeValue {
    fn default() -> Self {
        Self::String("".to_owned())
    }
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeValue::Null => write!(f, "null"),
            AttributeValue::Bool(v) => write!(f, "{}", v),
            AttributeValue::Number(v) => write!(f, "{}", v),
            AttributeValue::String(v) => write!(f, "{}", v),
            AttributeValue::Array(v) => write!(f, "{:?}", v),
            AttributeValue::Bytes(v) => write!(f, "{:?}", v),
            AttributeValue::Map(v) => write!(f, "{:?}", v),
            AttributeValue::DateTime(v) => write!(f, "{}", v),
        }
    }
}

impl From<serde_json::Value> for AttributeValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => AttributeValue::Null,
            serde_json::Value::Bool(v) => AttributeValue::Bool(v),
            serde_json::Value::Number(v) => AttributeValue::Number(v),
            serde_json::Value::String(v) => {
                if let Ok(v) = DateTime::try_from(v.as_str()) {
                    AttributeValue::DateTime(DateTime(v.into()))
                } else {
                    AttributeValue::String(v)
                }
            }
            serde_json::Value::Array(v) => {
                AttributeValue::Array(v.into_iter().map(AttributeValue::from).collect::<Vec<_>>())
            }
            serde_json::Value::Object(v) => AttributeValue::Map(
                v.into_iter()
                    .map(|(k, v)| (k, AttributeValue::from(v)))
                    .collect::<HashMap<_, _>>(),
            ),
        }
    }
}

impl From<AttributeValue> for serde_json::Value {
    fn from(value: AttributeValue) -> Self {
        match value {
            AttributeValue::Null => serde_json::Value::Null,
            AttributeValue::Bool(v) => serde_json::Value::Bool(v),
            AttributeValue::Number(v) => serde_json::Value::Number(v),
            AttributeValue::String(v) => serde_json::Value::String(v),
            AttributeValue::DateTime(v) => serde_json::Value::String(v.to_string()),
            AttributeValue::Array(v) => serde_json::Value::Array(
                v.into_iter()
                    .map(serde_json::Value::from)
                    .collect::<Vec<_>>(),
            ),
            AttributeValue::Bytes(v) => serde_json::Value::String(base64_encode(v.as_ref())),
            AttributeValue::Map(v) => serde_json::Value::Object(
                v.into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect::<serde_json::Map<_, _>>(),
            ),
        }
    }
}

impl From<nusamai_citygml::Value> for AttributeValue {
    fn from(value: nusamai_citygml::Value) -> Self {
        match value {
            nusamai_citygml::Value::String(v) => AttributeValue::String(v),
            nusamai_citygml::Value::Code(v) => AttributeValue::String(v.value().to_owned()),
            nusamai_citygml::Value::Integer(v) => AttributeValue::Number(Number::from(v)),
            nusamai_citygml::Value::NonNegativeInteger(v) => {
                AttributeValue::Number(Number::from(v))
            }
            nusamai_citygml::Value::Double(v) => {
                AttributeValue::Number(Number::from_f64(v).unwrap())
            }
            nusamai_citygml::Value::Measure(v) => {
                AttributeValue::Number(Number::from_f64(v.value()).unwrap())
            }
            nusamai_citygml::Value::Boolean(v) => AttributeValue::Bool(v),
            nusamai_citygml::Value::Uri(v) => AttributeValue::String(v.value().to_string()),
            nusamai_citygml::Value::Date(v) => {
                if let Ok(v) = DateTime::try_from(v.to_string()) {
                    AttributeValue::DateTime(v)
                } else {
                    AttributeValue::String(v.to_string())
                }
            }
            nusamai_citygml::Value::Point(v) => AttributeValue::Map(
                vec![
                    (
                        "type".to_string(),
                        AttributeValue::String("Point".to_string()),
                    ),
                    (
                        "coordinates".to_string(),
                        AttributeValue::Array(
                            v.coordinates()
                                .iter()
                                .map(|v| AttributeValue::Number(Number::from_f64(*v).unwrap()))
                                .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            nusamai_citygml::Value::Array(v) => {
                AttributeValue::Array(v.into_iter().map(AttributeValue::from).collect())
            }
            nusamai_citygml::Value::Object(v) => {
                let m = v
                    .attributes
                    .iter()
                    .map(|(k, v)| (k.into(), AttributeValue::from(v.clone())))
                    .collect();
                AttributeValue::Map(m)
            }
        }
    }
}

impl From<XmlXpathValue> for AttributeValue {
    fn from(value: XmlXpathValue) -> Self {
        std::convert::Into::<AttributeValue>::into(
            value.to_string().parse::<serde_json::Value>().unwrap(),
        )
    }
}

impl TryFrom<AttributeValue> for rhai::Dynamic {
    type Error = error::Error;

    fn try_from(value: AttributeValue) -> std::result::Result<Self, Self::Error> {
        let value: serde_json::Value = value.into();
        let value: rhai::Dynamic =
            serde_json::from_value(value).map_err(error::Error::internal_runtime)?;
        Ok(value)
    }
}

impl TryFrom<rhai::Dynamic> for AttributeValue {
    type Error = error::Error;

    fn try_from(value: rhai::Dynamic) -> std::result::Result<Self, Self::Error> {
        let value: serde_json::Value =
            from_dynamic(&value).map_err(error::Error::internal_runtime)?;
        let value: Self = value.into();
        Ok(normalize_action_value(value))
    }
}

fn normalize_action_value(value: AttributeValue) -> AttributeValue {
    match &value {
        AttributeValue::Map(v) => match v.len() {
            len if len > 1 => {
                let mut value = HashMap::new();
                for (k, v) in v.iter() {
                    value.insert(k.clone(), normalize_action_value(v.clone()));
                }
                AttributeValue::Map(value)
            }
            1 => {
                let (k, v) = v.iter().next().unwrap();
                match k.as_str() {
                    "String" => v.clone(),
                    "Number" => v.clone(),
                    _ => value,
                }
            }
            _ => value,
        },
        AttributeValue::Array(v) => {
            let result = v
                .iter()
                .map(|value| normalize_action_value(value.clone()))
                .collect::<Vec<_>>();
            AttributeValue::Array(result)
        }
        _ => value,
    }
}

impl TryFrom<Uri> for AttributeValue {
    type Error = error::Error;

    fn try_from(value: Uri) -> std::result::Result<Self, Self::Error> {
        let value: serde_json::Value =
            serde_json::to_value(value).map_err(error::Error::internal_runtime)?;
        Ok(value.into())
    }
}

impl Hash for AttributeValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            AttributeValue::Null => "Null".hash(state),
            AttributeValue::Bool(b) => {
                "Bool".hash(state);
                b.hash(state);
            }
            AttributeValue::Number(n) => {
                "Number".hash(state);
                n.hash(state);
            }
            AttributeValue::String(s) => {
                "String".hash(state);
                s.hash(state);
            }
            AttributeValue::Array(arr) => {
                "Array".hash(state);
                arr.hash(state);
            }
            AttributeValue::Bytes(b) => {
                "Bytes".hash(state);
                b.hash(state);
            }
            AttributeValue::Map(map) => {
                "Map".hash(state);
                for (k, v) in map {
                    k.hash(state);
                    v.hash(state);
                }
            }
            AttributeValue::DateTime(dt) => {
                "DateTime".hash(state);
                dt.hash(state);
            }
        }
    }
}

fn compare_numbers(n1: &Number, n2: &Number) -> Option<Ordering> {
    if let Some(i1) = n1.as_i64() {
        if let Some(i2) = n2.as_i64() {
            return i1.partial_cmp(&i2);
        }
    }
    if let Some(f1) = n1.as_f64() {
        if let Some(f2) = n2.as_f64() {
            return f1.partial_cmp(&f2);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_rhai_dynamic() {
        let dynamic_value = rhai::Dynamic::from(42);
        let action_value: std::result::Result<AttributeValue, _> = dynamic_value.try_into();
        assert_eq!(
            action_value.unwrap(),
            AttributeValue::Number(Number::from(42))
        );

        let dynamic_value = rhai::Dynamic::from("Hello");
        let action_value: std::result::Result<AttributeValue, _> = dynamic_value.try_into();
        assert_eq!(
            action_value.unwrap(),
            AttributeValue::String("Hello".to_string())
        );
    }

    #[test]
    fn test_partial_ord() {
        let number1 = AttributeValue::Number(Number::from(42));
        let number2 = AttributeValue::Number(Number::from(42));
        assert_eq!(number1.partial_cmp(&number2), Some(Ordering::Equal));

        let string1 = AttributeValue::String("Hello".to_string());
        let string2 = AttributeValue::String("World".to_string());
        assert_eq!(string1.partial_cmp(&string2), Some(Ordering::Less));
    }

    #[test]
    fn test_eq() {
        let number1 = AttributeValue::Number(Number::from(42));
        let number2 = AttributeValue::Number(Number::from(42));
        assert_eq!(number1, number2);

        let string1 = AttributeValue::String("Hello".to_string());
        let string2 = AttributeValue::String("Hello".to_string());
        assert_eq!(string1, string2);

        let map1 = AttributeValue::Map(
            vec![(
                "key".to_string(),
                AttributeValue::String("value".to_string()),
            )]
            .into_iter()
            .collect(),
        );
        let map2 = AttributeValue::Map(
            vec![(
                "key".to_string(),
                AttributeValue::String("value".to_string()),
            )]
            .into_iter()
            .collect(),
        );
        assert_eq!(map1, map2);
    }

    #[test]
    fn test_compare_numbers() {
        let number1 = Number::from(42);
        let number2 = Number::from(42);
        assert_eq!(compare_numbers(&number1, &number2), Some(Ordering::Equal));

        let number1 = Number::from(42);
        let number2 = Number::from(43);
        assert_eq!(compare_numbers(&number1, &number2), Some(Ordering::Less));

        let number1 = Number::from(43);
        let number2 = Number::from(42);
        assert_eq!(compare_numbers(&number1, &number2), Some(Ordering::Greater));
    }
}
