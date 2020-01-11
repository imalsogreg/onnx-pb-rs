include!(concat!(env!("OUT_DIR"), "/onnx.rs"));

impl From<i64> for tensor_shape_proto::dimension::Value {
    fn from(v: i64) -> tensor_shape_proto::dimension::Value {
        tensor_shape_proto::dimension::Value::DimValue(v)
    }
}

impl From<String> for tensor_shape_proto::dimension::Value {
    fn from(v: String) -> tensor_shape_proto::dimension::Value {
        tensor_shape_proto::dimension::Value::DimParam(v)
    }
}

impl<V: Into<tensor_shape_proto::dimension::Value>> From<V> for tensor_shape_proto::Dimension {
    fn from(v: V) -> tensor_shape_proto::Dimension {
        tensor_shape_proto::Dimension {
            denotation: "".to_owned(),
            value: Some(v.into()),
        }
    }
}
