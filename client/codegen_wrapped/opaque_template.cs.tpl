using IntPtr = System.IntPtr;
using generated = constellation.generated;
using RSharp;


namespace Teleportal.Client.{{namespace_super}}
{
    partial class ToManaged
    {
{{#if only_owned}}
        public static unsafe {{namespace_sub}}.{{class_ident}} f(Ptr<{{namespace_sub}}.{{class_ident}}> inner)
        {
            return new {{namespace_sub}}.{{class_ident}}(inner);
{{else}}
        public static unsafe {{namespace_sub}}.{{class_ident}} f(OwnershipSemantics ownershipSemantics, Ptr<{{namespace_sub}}.{{class_ident}}> inner)
        {
            return new {{namespace_sub}}.{{class_ident}}(inner, ownershipSemantics);
{{/if}}
        }
    }
}

namespace Teleportal.Client.{{namespace_super}}.{{namespace_sub}}
{
    public sealed class {{class_ident}} : OpaqueWrapper<{{class_ident}}>
    {
{{#if only_owned}}
        public unsafe {{class_ident}}(Ptr<{{class_ident}}> inner) : base(inner, OwnershipSemantics.Owned) { }
{{else}}
        public unsafe {{class_ident}}(Ptr<{{class_ident}}> inner, OwnershipSemantics ownershipSemantics) : base(inner, ownershipSemantics) { }
{{/if}}

{{#if new_expr}}
        public unsafe {{class_ident}}({{new_args}}) : base(
            new Ptr<{{class_ident}}>({{new_expr}}),
            OwnershipSemantics.Owned
        )
        { }
{{/if}}

        override protected void NativeDrop(Ptr<{{class_ident}}> inner)
        {
{{#if drop_ident}}
            {{drop_ident}}(inner.p);
{{/if}}
        }

        {{> additional_methods}}
    }
}
