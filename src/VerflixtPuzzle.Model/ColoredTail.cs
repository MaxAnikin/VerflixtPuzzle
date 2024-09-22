using System.Drawing;

namespace VerflixtPuzzle.Model
{
    internal class ColoredTail : ColoredSide
    {
        public ColoredTail(KnownColor color) : base(color)
        {
        }

        public override bool Fit(Side side)
        {
            var head = side as ColoredHead;

            if(head == null)
                return false;

            return Color == head.Color;
        }
    }
}
