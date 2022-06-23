public unsafe RBox_{{type_platform}} Value
{
    get
    {
        {
            var p = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
            var ptr = new Ptr<{{type_cs}}>((IntPtr)p);
            return new RBox_{{type_platform}}(ptr, OwnershipSemantics.SharedRef);
        }
    }
    set
    {
        {
            generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, value.NativePtr);
            value.Inner = null;
        }
    }
}
