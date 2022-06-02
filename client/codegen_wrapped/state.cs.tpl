public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
    set
    {
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, {{#unless is_ptr_type}}&{{/unless}}value);
    }
}
