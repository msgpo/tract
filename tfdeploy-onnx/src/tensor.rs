use tfdeploy::*;
use pb::*;

impl TfdFrom<TensorProto_DataType> for DatumType {
    fn tfd_from(t: &TensorProto_DataType) -> TfdResult<DatumType> {
        use self::TensorProto_DataType::*;
        match t {
            &BOOL => Ok(DatumType::Bool),
            &UINT8 => Ok(DatumType::U8),
            &INT8 => Ok(DatumType::I8),
            &INT32 => Ok(DatumType::I32),
            &FLOAT => Ok(DatumType::F32),
            &DOUBLE => Ok(DatumType::F64),
            &STRING => Ok(DatumType::String),
            _ => Err(format!("Unknown DatumType {:?}", t))?,
        }
    }

    /*
    fn to_onnx(&self) -> TfdResult<TensorProto_DataType> {
        use self::TensorProto_DataType::*;
        match self {
            DatumType::U8 => Ok(UINT8),
            DatumType::I8 => Ok(INT8),
            DatumType::I32 => Ok(INT32),
            DatumType::F32 => Ok(FLOAT),
            DatumType::F64 => Ok(DOUBLE),
            DatumType::String => Ok(STRING),
            DatumType::TDim => bail!("Dimension is not translatable in protobuf"),
        }
    }
    */
}

impl TfdFrom<TypeProto_Tensor> for TensorFact {
    fn tfd_from(t: &TypeProto_Tensor) -> TfdResult<TensorFact> {
        let mut fact = TensorFact::default();
        if t.has_elem_type() {
            fact = fact.with_datum_type(t.get_elem_type().to_tfd()?);
        }
        if t.has_shape() {
            let shape = t.get_shape();
            let shape: Vec<usize> = shape
                .get_dim()
                .iter()
                .map(|d| d.get_dim_value() as usize)
                .collect();
            fact = fact.with_shape(shape)
        }
        Ok(fact)
    }
}

impl TfdFrom<TensorProto> for Tensor {
    fn tfd_from(t: &TensorProto) -> TfdResult<Tensor> {
        let dt = t.get_data_type().to_tfd()?;
        let shape: Vec<usize> = t.get_dims().iter().map(|&i| i as usize).collect();
        if t.has_raw_data() {
            unsafe {
                match dt {
                    DatumType::F32 => Tensor::from_raw::<f32>(&*shape, t.get_raw_data()),
                    DatumType::Bool => {
                        Ok(Tensor::from_raw::<u8>(&*shape, t.get_raw_data())?.into_array::<u8>()?.mapv(|x| x != 0).into())
                    }
                    _ => unimplemented!("FIXME, tensor loading"),
                }
            }
        } else {
            match dt {
                DatumType::F32 => Tensor::f32s(&*shape, t.get_float_data()),
                DatumType::Bool => Ok(Tensor::i32s(&*shape, t.get_int32_data())?.into_array::<i32>()?.mapv(|x| x != 0).into()),
                _ => unimplemented!("FIXME"),
            }
        }
    }
}