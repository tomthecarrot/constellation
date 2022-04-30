use crate::{generate_class_data_generic, ClassData, ClassDataTemplate};

use indoc::indoc;

pub struct ChannelTemplate {}

impl ClassDataTemplate for ChannelTemplate {
    fn namespace_super() -> String {
        "Contract.Properties".to_string()
    }

    fn namespace_sub() -> String {
        "Channels".to_string()
    }

    fn class_ident() -> String {
        "Channel_<type_platform>".to_string()
    }

    fn new_args() -> String {
        "<type_cs> value, double time".to_string()
    }

    fn new_expr() -> Option<String> {
        None
    }

    fn drop_ident() -> String {
        "generated.__Internal.TpClientContractPropertiesChannelsChannel<type_platform>Drop"
            .to_string()
    }

    fn additional_methods() -> Option<String> {
        Some(indoc! {r#"
            public unsafe <type_cs> Value
            {
                get
                {
                    var result = generated.__Internal.TpClientContractPropertiesChannelsChannel<type_platform>Value(this.Ptr?.p ?? IntPtr.Zero);
                    return ToManaged.f(OwnershipSemantics.SharedRef, result);
                }
            }
        "#}.to_string())
    }

    fn generate_class_data() -> Vec<ClassData> {
        generate_class_data_generic::<Self>()
    }
}
