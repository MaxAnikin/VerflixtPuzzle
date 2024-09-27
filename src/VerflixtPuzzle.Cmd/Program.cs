// See https://aka.ms/new-console-template for more information

using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzleSimple();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

var positions = new HashSet<string>();

PermutateAndCheck(puzzle, 0);

void PermutateAndCheck(Puzzle puzzle1, int s)
{
    if (s + 1 == puzzle1.TilesCount)
    {
        RotateAndCheck(puzzle, 0);
        return;
    }

    PermutateAndCheck(puzzle1, s + 1);
}

Console.WriteLine("Search completed.");


void RotateAndCheck(Puzzle puzzle1, int i)
{
    for (int j = 0; j < 4; j++)
    {
        if (puzzle1.TilesCount > i + 1)
        {
            RotateAndCheck(puzzle1, i + 1);
        }

        if (puzzle1.IsSolved())
        {
            vis.Visualize(puzzle1);
            var positionId = puzzle1.GetPositionUniqueId();
            if (positions.Add(positionId))
                Console.WriteLine($"position: {positionId}");
        }

        puzzle1.GetTile(i).Rotate();
    }
}