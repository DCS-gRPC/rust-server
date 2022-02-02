/// Methods that can be used to serialize and deserialize [prost_types::Struct].
pub mod proto_struct {
    use std::collections::BTreeMap;
    use std::fmt;

    use prost_types::value::Kind;
    use prost_types::{ListValue, Struct, Value};
    use serde::de::{MapAccess, Visitor};
    use serde::ser::{SerializeMap, SerializeSeq};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(data: &Option<Struct>, se: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(data) = data {
            StructSe(data).serialize(se)
        } else {
            se.serialize_unit()
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Option<Struct>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<StructDe>::deserialize(de)?.map(|s| s.0))
    }

    /// Serializable Wrapper around [prost_types::Struct].
    struct StructSe<'a>(&'a Struct);

    impl<'a> Serialize for StructSe<'a> {
        fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = se.serialize_map(Some(self.0.fields.len()))?;
            for (key, val) in &self.0.fields {
                map.serialize_key(&key)?;
                map.serialize_value(&ValueSe(val))?;
            }
            map.end()
        }
    }

    /// Deserializable Wrapper around [prost_types::Struct].
    struct StructDe(Struct);

    impl<'de> Deserialize<'de> for StructDe {
        fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            de.deserialize_map(StructVisitor)
        }
    }

    /// Serializable Wrapper around [prost_types::Value].
    struct ValueSe<'a>(&'a Value);

    impl<'a> Serialize for ValueSe<'a> {
        fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if let Some(kind) = &self.0.kind {
                match kind {
                    Kind::NullValue(_) => se.serialize_unit(),
                    Kind::NumberValue(v) => v.serialize(se),
                    Kind::StringValue(v) => v.serialize(se),
                    Kind::BoolValue(v) => v.serialize(se),
                    Kind::StructValue(v) => StructSe(v).serialize(se),
                    Kind::ListValue(v) => {
                        let mut seq = se.serialize_seq(Some(v.values.len()))?;
                        for val in &v.values {
                            seq.serialize_element(&ValueSe(val))?
                        }
                        seq.end()
                    }
                }
            } else {
                se.serialize_none()
            }
        }
    }

    /// Serializable Wrapper around [prost_types::Value].
    struct ValueDe(Value);

    impl<'de> Deserialize<'de> for ValueDe {
        fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            de.deserialize_any(ValueVisitor)
        }
    }

    struct StructVisitor;

    impl<'de> Visitor<'de> for StructVisitor {
        type Value = StructDe;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("google.protobuf.Struct")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut fields = BTreeMap::new();
            while let Some((key, value)) = access.next_entry::<String, ValueDe>()? {
                fields.insert(key, value.0);
            }
            Ok(StructDe(Struct { fields }))
        }
    }

    struct ValueVisitor;

    impl ValueVisitor {
        fn visit_number<E>(self, v: impl TryInto<f64>) -> Result<ValueDe, E>
        where
            E: serde::de::Error,
        {
            v.try_into()
                .map(|v| {
                    ValueDe(Value {
                        kind: Some(Kind::NumberValue(v)),
                    })
                })
                .map_err(|_| {
                    serde::de::Error::invalid_type(serde::de::Unexpected::Other("f64"), &self)
                })
        }
    }

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = ValueDe;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("google.protobuf.Value")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(ValueDe(Value {
                kind: Some(Kind::BoolValue(v)),
            }))
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            i32::try_from(v)
                .map_err(|_| {
                    serde::de::Error::invalid_type(serde::de::Unexpected::Other("i64"), &self)
                })
                .and_then(|v| self.visit_number(v))
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            i32::try_from(v)
                .map_err(|_| {
                    serde::de::Error::invalid_type(serde::de::Unexpected::Other("i64"), &self)
                })
                .and_then(|v| self.visit_number(v))
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u32::try_from(v)
                .map_err(|_| {
                    serde::de::Error::invalid_type(serde::de::Unexpected::Other("i64"), &self)
                })
                .and_then(|v| self.visit_number(v))
        }

        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u32::try_from(v)
                .map_err(|_| {
                    serde::de::Error::invalid_type(serde::de::Unexpected::Other("i64"), &self)
                })
                .and_then(|v| self.visit_number(v))
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_number(v)
        }

        fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(ValueDe(Value {
                kind: Some(Kind::StringValue(v.to_string())),
            }))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(ValueDe(Value {
                kind: Some(Kind::StringValue(v.to_string())),
            }))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(ValueDe(Value {
                kind: Some(Kind::NullValue(0)),
            }))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(val) = seq.next_element::<ValueDe>()? {
                values.push(val.0);
            }

            Ok(ValueDe(Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            }))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Ok(ValueDe(Value {
                kind: Some(Kind::StructValue(StructVisitor.visit_map(map)?.0)),
            }))
        }
    }

    #[cfg(test)]
    mod tests {
        use prost_types::value::Kind;
        use prost_types::{ListValue, Struct, Value};

        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Message {
            #[serde(with = "crate::utils::proto_struct")]
            details: Option<Struct>,
        }

        fn create_message(key: impl Into<String>, kind: Kind) -> Message {
            Message {
                details: Some(Struct {
                    fields: [(key.into(), Value { kind: Some(kind) })]
                        .into_iter()
                        .collect(),
                }),
            }
        }

        #[test]
        fn test_null() {
            let m = create_message("null", Kind::NullValue(0));
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(json, r#"{"details":{"null":null}}"#);
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }

        #[test]
        fn test_number() {
            let m = create_message("number", Kind::NumberValue(42.2223));
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(json, r#"{"details":{"number":42.2223}}"#);
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }

        #[test]
        fn test_string() {
            let m = create_message("string", Kind::StringValue("dcs-grpc".to_string()));
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(json, r#"{"details":{"string":"dcs-grpc"}}"#);
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }

        #[test]
        fn test_bool() {
            let m = create_message("bool", Kind::BoolValue(true));
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(json, r#"{"details":{"bool":true}}"#);
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }

        #[test]
        fn test_struct() {
            let m = create_message(
                "nested",
                Kind::StructValue(Struct {
                    fields: [
                        (
                            "number".to_string(),
                            Value {
                                kind: Some(Kind::NumberValue(42.0)),
                            },
                        ),
                        (
                            "string".to_string(),
                            Value {
                                kind: Some(Kind::StringValue("dcs-grpc".to_string())),
                            },
                        ),
                    ]
                    .into_iter()
                    .collect(),
                }),
            );
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(
                json,
                r#"{"details":{"nested":{"number":42.0,"string":"dcs-grpc"}}}"#
            );
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }

        #[test]
        fn test_list() {
            let m = create_message(
                "list",
                Kind::ListValue(ListValue {
                    values: vec![
                        Value {
                            kind: Some(Kind::NumberValue(42.0)),
                        },
                        Value {
                            kind: Some(Kind::StringValue("dcs-grpc".to_string())),
                        },
                    ],
                }),
            );
            let json = serde_json::to_string(&m).unwrap();
            assert_eq!(json, r#"{"details":{"list":[42.0,"dcs-grpc"]}}"#);
            assert_eq!(serde_json::from_str::<Message>(&json).unwrap(), m);
        }
    }
}
