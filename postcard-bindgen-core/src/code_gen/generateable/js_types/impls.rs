use genco::{prelude::js::Tokens, quote};

use crate::{
    type_info::{
        bool_to_js_bool, ArrayMeta, JsType, NumberMeta, ObjectMeta, OptionalMeta, StringMeta,
    },
    utils::StrExt,
};

use super::{
    des, ser,
    ty_check::{self, AvailableCheck},
    AccessorGenerateable,
};

impl AccessorGenerateable for JsType {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ser_accessor(field_access, field_accessor),
            Self::Array(array_meta) => array_meta.gen_ser_accessor(field_access, field_accessor),
            Self::Object(object_meta) => object_meta.gen_ser_accessor(field_access, field_accessor),
            Self::Optional(optional_meta) => {
                optional_meta.gen_ser_accessor(field_access, field_accessor)
            }
            Self::String(string_meta) => string_meta.gen_ser_accessor(field_access, field_accessor),
        }
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_des_accessor(field_accessor),
            Self::Array(array_meta) => array_meta.gen_des_accessor(field_accessor),
            Self::Object(object_meta) => object_meta.gen_des_accessor(field_accessor),
            Self::Optional(optional_meta) => optional_meta.gen_des_accessor(field_accessor),
            Self::String(string_meta) => string_meta.gen_des_accessor(field_accessor),
        }
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ty_check(field_access, inner_access),
            Self::Array(array_meta) => array_meta.gen_ty_check(field_access, inner_access),
            Self::Object(object_meta) => object_meta.gen_ty_check(field_access, inner_access),
            Self::Optional(optional_meta) => optional_meta.gen_ty_check(field_access, inner_access),
            Self::String(string_meta) => string_meta.gen_ty_check(field_access, inner_access),
        }
    }
}

impl AccessorGenerateable for OptionalMeta {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        let type_accessor = self.inner.gen_ser_accessor(field_access, field_accessor);
        quote!(if (v$field_access$field_accessor !== undefined) { s.serialize_number(U32_BYTES, false, 1); $type_accessor } else { s.serialize_number(U32_BYTES, false, 0) })
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let inner_accessor = self.inner.gen_des_accessor(des::FieldAccessor::None);
        quote!($(field_accessor)(d.deserialize_number(U32_BYTES, false) === 0) ? undefined : $inner_accessor)
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        let available_check = ty_check::AvailableCheck::from_field_access_and_inner_type_access(
            field_access,
            inner_access,
        );
        let inner_type_check = self.inner.gen_ty_check(field_access, inner_access);
        match &available_check {
            AvailableCheck::Object(_, _) => {
                quote!((($(available_check.to_owned()) && (v$inner_access$field_access !== undefined && $inner_type_check) || v$inner_access$field_access === undefined) || !($available_check)))
            }
            AvailableCheck::None => {
                quote!((v$inner_access$field_access !== undefined && $inner_type_check) || v$inner_access$field_access === undefined)
            }
        }
    }
}

impl AccessorGenerateable for StringMeta {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        quote!(s.serialize_string(v$field_access$field_accessor))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        quote!($(field_accessor)d.deserialize_string())
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        quote!(typeof v$inner_access$field_access === "string")
    }
}

impl AccessorGenerateable for ObjectMeta {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!(serialize_$obj_ident(s, v$field_access$field_accessor))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!($(field_accessor)deserialize_$obj_ident(d))
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!(is_$obj_ident(v$inner_access$field_access))
    }
}

impl AccessorGenerateable for ArrayMeta {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        let inner_type_accessor = self
            .items_type
            .gen_ser_accessor(ser::InnerTypeAccess::Direct, ser::FieldAccessor::Direct);
        quote!(s.serialize_array((s, v) => $inner_type_accessor, v$field_access$field_accessor))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let inner_type_accessor = self.items_type.gen_des_accessor(des::FieldAccessor::Array);
        quote!($(field_accessor)d.deserialize_array(() => $inner_type_accessor))
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        quote!(Array.isArray(v$inner_access$field_access))
    }
}

impl AccessorGenerateable for NumberMeta {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens {
        let byte_amount_str = self.as_byte_js_string();
        let signed = bool_to_js_bool(self.signed);
        quote!(s.serialize_number($byte_amount_str, $signed, v$field_access$field_accessor))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let byte_amount_str = self.as_byte_js_string();
        let signed = bool_to_js_bool(self.signed);
        quote!($(field_accessor)d.deserialize_number($byte_amount_str, $signed))
    }

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens {
        quote!(typeof v$inner_access$field_access === "number")
    }
}
