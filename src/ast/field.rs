use super::ty;

type Field = Box<Field_>;
pub type FieldList = Vec<Field>;

pub struct Field_ {
    pos: (i32, i32),
    var: String,
    ty: ty::Type,
    escape: bool,
}

impl Field_ {
    fn newField(pos: (i32, i32), var: String, ty: ty::Type, escape: bool) -> Field {
        Box::new(Field_ {
            pos,
            var,
            ty,
            escape,
        })
    }
}
