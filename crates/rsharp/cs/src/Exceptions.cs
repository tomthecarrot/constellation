namespace RSharp
{
    /// Thrown whenever immutable data is used mutably
    [System.Serializable]
    public class MutabilityException : System.Exception
    {
        public MutabilityException() { }
        public MutabilityException(string message) : base(message) { }
        public MutabilityException(string message, System.Exception inner) : base(message, inner) { }
        protected MutabilityException(
            System.Runtime.Serialization.SerializationInfo info,
            System.Runtime.Serialization.StreamingContext context) : base(info, context) { }
    }

    /// Thrown whenever ownership is required but only a reference is held
    [System.Serializable]
    public class OwnershipException : System.Exception
    {
        public OwnershipException() { }
        public OwnershipException(string message) : base(message) { }
        public OwnershipException(string message, System.Exception inner) : base(message, inner) { }
        protected OwnershipException(
            System.Runtime.Serialization.SerializationInfo info,
            System.Runtime.Serialization.StreamingContext context) : base(info, context) { }
    }
}
