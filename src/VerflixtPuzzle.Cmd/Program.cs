// See https://aka.ms/new-console-template for more information
using BenchmarkDotNet.Running;
using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzle();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();


//BenchmarkRunner.Run<PuzzleBenchmark>();

//RotateAndCheck(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

Permutate(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8], 9, ((puzzle1, arr) => RotateAndCheck(puzzle1, arr)));

Console.WriteLine("Search completed.");

void Permutate(Puzzle puzzle1, int[] arr, int size, Action<Puzzle, int[]> action)
{
    if (size == 1)
    {
        action(puzzle1, arr);
        return;
    }

    for (int i = 0; i < size; i++)
    {
        Permutate(puzzle1, arr, size - 1, action);

        if (size % 2 == 0)
        {
            (arr[i], arr[size - 1]) = (arr[size - 1], arr[i]);
            puzzle1.Swap(arr[i], arr[size - 1]);
        }
        else
        {
            (arr[0], arr[size - 1]) = (arr[size - 1], arr[0]);
            puzzle1.Swap(arr[0], arr[size - 1]);
        }
    }
}

void RotateAndCheck(Puzzle puzzle1, int[] tiles)
{
    for (int j = 0; j < 4; j++)
    {
        if (puzzle1.TilesCount > i + 1)
        {
            RotateAndCheck(puzzle1, order, i + 1);
        }

        if (puzzle.IsSolved)
        {
            Console.WriteLine($"Found solution: {string.Join(",", order)}");
            vis.Visualize(puzzle1, order);
        }

        puzzle1.GetTile(order[i]).Rotate();
    }
}

//static void Swap(ref int a, ref int b)
//{
//    (a, b) = (b, a);
//}

//void Permutate(Puzzle puzzle1, int[] arr, int size, Action<Puzzle, int[]> action)
//{
//    if (size == 1)
//    {
//        action(puzzle1, arr);
//        return;
//    }

//    for (int i = 0; i < size; i++)
//    {
//        Permutate(puzzle1, arr, size - 1, action);

//        if (size % 2 == 0)
//        {
//            Swap(ref arr[i], ref arr[size - 1]);
//        }
//        else
//        {
//            Swap(ref arr[0], ref arr[size - 1]);
//        }
//    }
//}


//void RotateAndCheck(Puzzle puzzle1, int[] order, int i = 0)
//{
//    for (int j = 0; j < 4; j++)
//    {
//        if (puzzle1.TilesCount > i + 1)
//        {
//            RotateAndCheck(puzzle1, order, i + 1);
//        }

//        if (new DefaultIsSolvedStrategy(order).IsSolved(puzzle1))
//        {
//            Console.WriteLine($"Found solution: {string.Join(",", order)}");
//            vis.Visualize(puzzle1, order);
//        }

//        puzzle1.GetTile(order[i]).Rotate();
//    }
//}