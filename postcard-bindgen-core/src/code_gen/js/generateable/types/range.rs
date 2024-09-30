use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::js::{
        generateable::{VariableAccess, VariablePath},
        utils::semicolon_chain,
    },
    type_info::RangeMeta,
};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for RangeMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let start_path = variable_path
            .to_owned()
            .modify_push(VariableAccess::Field("start".into()));
        let stop_path = variable_path.modify_push(VariableAccess::Field("end".into()));

        let start_accessor = self.bounds_type.gen_ser_accessor(start_path);
        let stop_accessor = self.bounds_type.gen_ser_accessor(stop_path);

        semicolon_chain([start_accessor, stop_accessor])
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let field_des = self.bounds_type.gen_des_accessor(des::FieldAccessor::None);
        quote!($field_accessor{ start: $(field_des.to_owned()), end: $field_des })
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(typeof $(variable_path.to_owned()) === "object" && "start" in $(variable_path.to_owned()) && "end" in $variable_path)
    }

    fn gen_ts_type(&self) -> Tokens {
        quote!({ start: $(self.bounds_type.gen_ts_type()), end: $(self.bounds_type.gen_ts_type()) })
    }
}
