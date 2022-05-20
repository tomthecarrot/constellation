// This file is autogenerated, with one per class.

using IntPtr = System.IntPtr;
using generated = tp_client.generated;
using RSharp;


namespace Teleportal.Client.{{namespace_super}}
{
    partial class ToManaged
    {
        public static unsafe {{namespace_sub}}.{{class_ident}} f(OwnershipSemantics ownershipSemantics, Ptr<{{namespace_sub}}.{{class_ident}}> inner)
        {
            return new {{namespace_sub}}.{{class_ident}}(inner, ownershipSemantics);
        }
    }
}

namespace Teleportal.Client.{{namespace_super}}.{{namespace_sub}}
{
    public sealed class {{class_ident}} : OpaqueWrapper<{{class_ident}}>
    {
        public {{class_ident}}(Ptr<{{class_ident}}> inner, OwnershipSemantics ownershipSemantics) : base(inner, ownershipSemantics) { }

        {{#if new_expr}}
        public unsafe {{class_ident}}({{new_args}}) : base(
            new Ptr<{{class_ident}}>({{new_expr}}),
            OwnershipSemantics.Owned
        )
        { }
        {{/if}}

        override protected void NativeDrop(Ptr<{{class_ident}}> inner)
        {
            {{drop_ident}}(inner.p);
        }

        {{> additional_methods}}
    }
}
