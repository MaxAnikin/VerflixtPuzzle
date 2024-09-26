// See https://aka.ms/new-console-template for more information

using VerflixtPuzzle.Cmd;

var c = Console.GetCursorPosition();

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzle();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

int overall = 0;
int cur = 0;
while (!puzzle.IsSolved() || overall < 500)
{
    if (cur > 3)
    {
        puzzle.GetTile(overall / 4).Rotate();
        cur = 0;
        vis.Visualize(puzzle);
    }

    puzzle.GetTile(current).Rotate();
    vis.Visualize(puzzle);

    cur++;
    overall++;
} 