// See https://aka.ms/new-console-template for more information

using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzle();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle);


Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

Permutate(new Queue<int>(Enumerable.Range(0, 9)), new Stack<int>());

void Permutate(Queue<int> nums, Stack<int> stack)
{
    for (int i = 0; i < nums.Count; i++)
    {
        if (nums.TryDequeue(out int cur))
        {
            stack.Push(cur);
            Permutate(nums, stack);
            stack.Pop();
            nums.Enqueue(cur);
        }
    }

    if (nums.Count == 0)
    {
        var order = stack.ToArray();
        puzzle.Permutate(order);
        RotateAndCheck(puzzle, 0);
    }
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
            Console.WriteLine($"position: {positionId}");
            Console.WriteLine();
        }

        puzzle1.GetTile(i).Rotate();
    }
}