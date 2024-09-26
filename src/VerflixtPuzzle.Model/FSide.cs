namespace VerflixtPuzzle.Model.FSide
{
    public record Side(Dictionary<SideAttributeType, int> Attributes)
    {
        public bool Fit(Side side)
        {
            if (side == null) throw new ArgumentNullException(nameof(side));

            if (side.Attributes.Count != Attributes.Count)
                throw new InvalidOperationException("Different attributes");

            foreach (var thisAttribute in Attributes)
            {
                if(!side.Attributes.TryGetValue(thisAttribute.Key, out var value))
                    throw new InvalidOperationException($"Missing attribute: {thisAttribute.Key}");

                if (!thisAttribute.Key.Fit(thisAttribute.Value, value))
                    return false;
            }

            return true;
        }
    }

    public abstract record SideAttributeType(string Name)
    {
        public abstract bool Fit(int v1, int v2);
    }

    public record XorSideAttributeType(string Name, int Mask) : SideAttributeType(Name)
    {
        public override bool Fit(int v1, int v2) => ((v1 ^ v2) & Mask) == 0;
    }

    public record EqualitySideAttributeType(string Name) : SideAttributeType(Name)
    {
        public override bool Fit(int v1, int v2) => v1 == v2;
    }
}