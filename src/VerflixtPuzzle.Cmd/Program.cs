// See https://aka.ms/new-console-template for more information
using BenchmarkDotNet.Running;
using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzle();

//int[] order = [0, 1, 2, 3, 4, 5, 8, 7, 6];
int[] order = Enumerable.Range(0, 9).ToArray();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle, order);
Console.WriteLine();


// BenchmarkRunner.Run<PuzzleBenchmark>();

//Permutate(new Queue<int>(Enumerable.Range(9, 0)), new Stack<int>());

//var orderedRange = Enumerable.Range(0, 9).OrderBy(i => i);
//var queue = new Queue<int>(orderedRange);

//PermutateRec(null, null, (res) => Console.WriteLine(string.Join(",", res)));

//ShowPermutations(queue, new Stack<int>());

var strategy = new OrderBasedStrategy();

 PermutateRec(null, null, (order) => RotateAndCheck(puzzle, order));


//int i = 0;
//PermutateRec(null, null, (order) => i++);
//Console.WriteLine($"Permutations count: {i}");

void PermutateRec(Queue<int>? nums, Stack<int>? res, Action<int[]> permutationAction)
{
    nums ??= new Queue<int>(Enumerable.Range(0, 9).OrderByDescending(t => t));
    res ??= new Stack<int>(nums.Count);

    if (nums.Count == 0)
    {
        permutationAction(res.ToArray());
        return;
    }

    for (int i = 0; i < nums.Count; i++)
    {
        int cur = nums.Dequeue();
        res.Push(cur);
        PermutateRec(nums, res, permutationAction);
        res.Pop();
        nums.Enqueue(cur);
    }
}

void PermutateLoop(int[] range)
{
    for (int i = 0; i < range.Length; i++)
    {

    }
}

void ShowPermutations(Queue<int> nums, Stack<int> stack)
{
    for (int i = 0; i < nums.Count; i++)
    {
        if (nums.TryDequeue(out int cur))
        {
            stack.Push(cur);
            ShowPermutations(nums, stack);
            stack.Pop();
            nums.Enqueue(cur);
        }
    }

    if (nums.Count == 0)
    {
        var order = stack.ToArray();
        Console.WriteLine(string.Join(",", order));
    }
}

//void Permutate(Queue<int> nums, Stack<int> stack)
//{
//    for (int i = 0; i < nums.Count; i++)
//    {
//        if (nums.TryDequeue(out int cur))
//        {
//            stack.Push(cur);
//            Permutate(nums, stack);
//            stack.Pop();
//            nums.Enqueue(cur);
//        }
//    }

//    if (nums.Count == 0)
//    {
//        var order = stack.ToArray();
//        RotateAndCheck(puzzle, order, 0);
//    }
//}

Console.WriteLine("Search completed.");

void RotateAndCheck(Puzzle puzzle1, int[] order, int i = 0)
{
    for (int j = 0; j < 4; j++)
    {
        if (puzzle1.TilesCount > i + 1)
        {
            RotateAndCheck(puzzle1, order, i + 1);
        }

        if (strategy.IsSolved(puzzle1, order))
        {
            Console.WriteLine($"Found solution: {string.Join(",", order)}");
            vis.Visualize(puzzle1, order);
        }

        puzzle1.GetTile(order[i]).Rotate();
    }
}