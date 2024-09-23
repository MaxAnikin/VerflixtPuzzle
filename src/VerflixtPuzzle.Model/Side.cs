using System.Runtime.CompilerServices;

namespace VerflixtPuzzle.Model
{
    public enum SideTypeOperator
    {
        Xor,
        And
    }

    public abstract record SideType;

    public abstract record XorSideType();

    public record Side(XorSideType Xor, int And)
    {
        public bool Fit(Side s2)
        {
            return 
        }
    }

    public static class SideExtensions
    {
        public static bool Fit(this Side s1, Side s2)
        {
            s1.
        }
    }
}