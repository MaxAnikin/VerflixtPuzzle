using System.Drawing;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;
using VerflixtPuzzle.Model.TSide;

public class PuzzleBuilder
{
    public Puzzle BuildTPuzzleSimple()
    {
        var tiles = new SortedList<int, SquareTile>
        {
            // 0-2
            { 0,
                new SquareTile([new TortoiseSide(Shape.Tail, Color.Blue),
                    new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Red)])
            },

            { 1,
                new SquareTile(
                    [new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)])
            },

            { 2
                , new SquareTile(
                    [new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)])
            },

            // 3-5
            { 3
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
            { 4
                , new SquareTile(
                    [new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)])
            },
            { 5
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
        
            // 6-8
            {6,
                new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
            {7
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
            {8
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)])
            }
        };

        return new Puzzle(tiles.Select(s => s.Value).ToArray());
    }

    public Puzzle BuildTPuzzle()
    {
        var tiles = new SortedList<int, SquareTile>
        {
            // 0-2
            { 0,
                new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Tail, Color.Green)])
            },

            { 1,
                new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Blue)])
            },

            { 2
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },

            // 3-5
            { 3
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
            { 4
                , new SquareTile(
                    [new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Red)
                    , new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red)])
            },
            { 5
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Blue)])
            },
        
            // 6-8
            {6,
                new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Red)])
            },
            {7
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Red)])
            },
            {8
                , new SquareTile(
                    [new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Yellow)])
            }
        };

        return new Puzzle(tiles.Select(s => s.Value).ToArray());
    }
}