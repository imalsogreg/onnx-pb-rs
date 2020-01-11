include!(concat!(env!("OUT_DIR"), "/onnx.rs"));

impl From<i64> for tensor_shape_proto::dimension::Value {
    fn from(v: i64) -> Self {
        tensor_shape_proto::dimension::Value::DimValue(v)
    }
}

impl From<String> for tensor_shape_proto::dimension::Value {
    fn from(v: String) -> Self {
        tensor_shape_proto::dimension::Value::DimParam(v)
    }
}

impl<V: Into<tensor_shape_proto::dimension::Value>> From<V> for tensor_shape_proto::Dimension {
    fn from(v: V) -> Self {
        tensor_shape_proto::Dimension {
            denotation: "".to_owned(),
            value: Some(v.into()),
        }
    }
}

impl<K: Into<String>, V: Into<String>> From<(K, V)> for StringStringEntryProto {
    fn from((k, v): (K, V)) -> Self {
        StringStringEntryProto {
            key: k.into(),
            value: v.into(),
        }
    }
}
