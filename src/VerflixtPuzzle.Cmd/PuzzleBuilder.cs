using System.Drawing;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.TSide;

public class PuzzleBuilder
{
    public Puzzle BuildTPuzzle()
    {
        var tiles = new SortedList<int, Tile>
        {
            // 0-2
            { 0,
                new Tile(
                    new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Tail, Color.Green))
            },

            { 1,
                new Tile(
                    new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Blue))
            },

            { 2
                , new Tile(
                    new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue))
            },

            // 3-5
            { 3
                , new Tile(
                    new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Blue))
            },
            { 4
                , new Tile(
                    new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Red)
                    , new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Red))
            },
            { 5
                , new Tile(
                    new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Blue))
            },
        
            // 6-8
            {6,
                new Tile(
                    new TortoiseSide(Shape.Tail, Color.Yellow)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Green)
                    , new TortoiseSide(Shape.Tail, Color.Red))
            },
            {7
                , new Tile(
                    new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Head, Color.Yellow)
                    , new TortoiseSide(Shape.Tail, Color.Red))
            },
            {8
                , new Tile(
                    new TortoiseSide(Shape.Tail, Color.Green)
                    , new TortoiseSide(Shape.Head, Color.Red)
                    , new TortoiseSide(Shape.Head, Color.Blue)
                    , new TortoiseSide(Shape.Tail, Color.Yellow))
            }
        };

        return new Puzzle(tiles);
    }
}