public unsafe {{value_owned_ident}} Value
{
    get
    {
        var raw_ptr = generated.__Internal.TpClientContractPropertiesStatesState{{value_mangled_name}}Value(this.Inner.Value.p);
        var ptr = new Ptr<{{value_ptr_inner}}>((IntPtr) raw_ptr);
{{#if has_second_arg}}
        return new {{value_owned_ident}}(ptr, OwnershipSemantics.SharedRef);
{{else}}
        // TODO[SER-389]: This violates rust's single ownership rule, and is unsound
        return new {{value_owned_ident}}(ptr);
{{/if}}
    }

    set
    {
        {
            generated.__Internal.TpClientContractPropertiesStatesState{{value_mangled_name}}ValueSet(this.Inner.Value.p, ({{value_ptr_raw}}) value.StealInner().p);
            value.Inner = null;
        }
    }
}
