// See https://aka.ms/new-console-template for more information

using BenchmarkDotNet.Running;
using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model.Puzzle;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzleSimple();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

//GeneratePermutations([0, 1, 2, 3, 4, 5, 6, 7, 8], 9);

//BenchmarkRunner.Run<PuzzleBenchmark>();


Permutate(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8], 9);

//var orderedRange = Enumerable.Range(0, 9).OrderBy(i => i);
//var queue = new Queue<int>(orderedRange);

//PermutateRec(null, null);

//ShowPermutations(queue, new Stack<int>());

//RotateAndCheck(puzzle, [0, 1, 2,3,4,5,6,7,8], 0);

static void GeneratePermutations(int[] arr, int size)
{
    if (size == 1)
    {
        Console.WriteLine(string.Join(", ", arr));
        return;
    }

    // Генерируем перестановки рекурсивно
    for (int i = 0; i < size; i++)
    {
        GeneratePermutations(arr, size - 1);

        // Если размер подмассива четный, меняем местами i-й и последний элемент
        if (size % 2 == 0)
        {
            Swap(ref arr[i], ref arr[size - 1]);
        }
        // Если размер подмассива нечетный, меняем первый и последний элементы
        else
        {
            Swap(ref arr[0], ref arr[size - 1]);
        }
    }
}

static void Swap(ref int a, ref int b)
{
    int temp = a;
    a = b;
    b = temp;
}

void PermutateRec(Queue<int>? nums, Stack<int>? res)
{
    nums ??= new Queue<int>(Enumerable.Range(0, 9).OrderByDescending(t => t));
    res ??= new Stack<int>(nums.Count);

    if (nums.Count == 0)
        return;

    for (int i = 0; i < nums.Count; i++)
    {
        int cur = nums.Dequeue();
        res.Push(cur);
        PermutateRec(nums, res);
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

void Permutate(Puzzle puzzle1, int[] arr, int size)
{
    if (size == 1)
    {
        RotateAndCheck(puzzle1, arr, 0);
        return;
    }

    // Генерируем перестановки рекурсивно
    for (int i = 0; i < size; i++)
    {
        Permutate(puzzle1, arr, size - 1);

        // Если размер подмассива четный, меняем местами i-й и последний элемент
        if (size % 2 == 0)
        {
            Swap(ref arr[i], ref arr[size - 1]);
        }
        // Если размер подмассива нечетный, меняем первый и последний элементы
        else
        {
            Swap(ref arr[0], ref arr[size - 1]);
        }
    }
}

Console.WriteLine("Search completed.");

void RotateAndCheck(Puzzle puzzle1, int[] order, int i)
{
    for (int j = 0; j < 4; j++)
    {
        if (puzzle1.TilesCount > i + 1)
        {
            RotateAndCheck(puzzle1, order, i + 1);
        }

        if (puzzle1.IsSolved(order))
        {
            //vis.Visualize(puzzle1, order);
            var positionId = puzzle1.GetPositionUniqueId(order);
            Console.WriteLine($"position: {positionId}");
            Console.WriteLine();
        }

        puzzle1.GetTile(order[i]).Rotate();
    }
}