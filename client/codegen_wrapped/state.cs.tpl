public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
    set
    {
        var boxed_value = RBox_{{type_platform}}.new_(value);
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, boxed_value);
    }
}
