public unsafe {{value_owned_ident}} Value
{
    get
    {
        var raw_ptr = generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{value_mangled_name}}Value(this.Inner.Value.p);
        var ptr = new Ptr<{{value_ptr_inner}}>((IntPtr) raw_ptr);
{{#if has_second_arg}}
        return new {{value_owned_ident}}(ptr, OwnershipSemantics.SharedRef);
{{else}}
        return new {{value_owned_ident}}(ptr);
{{/if}}
    }
}

public double Time
{
    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{value_mangled_name}}Time(this.Inner.Value.p);
}
