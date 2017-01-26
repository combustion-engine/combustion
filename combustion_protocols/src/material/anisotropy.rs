use nalgebra::Vector3;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAnisotropy {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub rotation: Option<Vector3<f32>>,
}

impl MaterialAnisotropy {
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.amount.is_none() && self.rotation.is_none()
    }

    #[inline(always)]
    pub fn from_amount(amount: f32) -> MaterialAnisotropy {
        MaterialAnisotropy { amount: Some(amount), rotation: None }
    }
}

impl Default for MaterialAnisotropy {
    #[inline(always)]
    fn default() -> MaterialAnisotropy {
        MaterialAnisotropy { amount: None, rotation: None }
    }
}

pub mod de {
    use serde::de::{self, Deserialize, Deserializer};

    use std::fmt;

    use super::*;

    pub fn from_num_or_value<D>(d: D) -> Result<MaterialAnisotropy, D::Error> where D: Deserializer {
        struct NumOrAnisotropyVisitor;

        macro_rules! impl_visit_num {
            ($name:ident, $t:ty) => {
                fn $name<E>(self, value: $t) -> Result<MaterialAnisotropy, E> where E: de::Error {
                    Ok(MaterialAnisotropy::from_amount(value as f32))
                }
            }
        }

        impl de::Visitor for NumOrAnisotropyVisitor {
            type Value = MaterialAnisotropy;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "number or anisotropy structure")
            }

            fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error> where M: de::MapVisitor {
                Deserialize::deserialize(de::value::MapVisitorDeserializer::new(visitor))
            }

            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error> where V: de::SeqVisitor {
                Deserialize::deserialize(de::value::SeqVisitorDeserializer::new(visitor))
            }

            impl_visit_num!(visit_i8, i8);
            impl_visit_num!(visit_u8, u8);

            impl_visit_num!(visit_i16, i16);
            impl_visit_num!(visit_u16, u16);

            impl_visit_num!(visit_i32, i32);
            impl_visit_num!(visit_u32, u32);

            impl_visit_num!(visit_i64, i64);
            impl_visit_num!(visit_u64, u64);

            impl_visit_num!(visit_f32, f32);
            impl_visit_num!(visit_f64, f64);
        }

        d.deserialize(NumOrAnisotropyVisitor)
    }
}