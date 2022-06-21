public RBox_{{type_platform}} Value
{
    get
    {
        unsafe
        {
            var p = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
            Ptr<{{type_cs}}> ptr = new Ptr<{{type_cs}}>((IntPtr)p);
            return new RBox_{{type_platform}}(ptr, OwnershipSemantics.SharedRef);
        }
    }
    set
    {
        unsafe
        {
            generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, value.NativePtr);
            value.Inner = null;
        }
    }
}
