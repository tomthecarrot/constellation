public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner?.p ?? IntPtr.Zero);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
    set
    {
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner?.p ?? IntPtr.Zero, {{#unless is_ptr_type}}&{{/unless}}value);
    }
}
