use super::{exp::Exp, field};

pub type TypeList = Vec<Box<Type_>>;
pub type Type = Box<Type_>;

#[derive(Clone, Debug)]
pub struct Type_ {
    pub pos: u32,
    pub data: TypeData,
}

#[derive(Clone, Debug)]
pub enum TypeData {
    Void,
    Name(String),
    Array(Type, Exp),
    Pointer(Type),
    Template(String, Vec<Type>),
    Func(field::FieldList, Type),
    None,
}

impl Type_ {
    pub fn void_type(pos: u32) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Void,
        })
    }
    pub fn name_type(pos: u32, name: String) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Name(name),
        })
    }
    pub fn array_type(pos: u32, ty: Type, size: Exp) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Array(ty, size),
        })
    }
    pub fn pointer_type(pos: u32, ty: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Pointer(ty),
        })
    }
    pub fn template_type(pos: u32, name: String, ty_params: Vec<Type>) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Template(name, ty_params),
        })
    }
    pub fn func_type(pos: u32, params: field::FieldList, result: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Func(params, result),
        })
    }
}
