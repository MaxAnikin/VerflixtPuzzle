using System.Collections.Concurrent;
using BenchmarkDotNet.Attributes;
using VerflixtPuzzle.Model;

namespace VerflixtPuzzle.Cmd
{
    [ThreadingDiagnoser]
    [MemoryDiagnoser]
    public class PuzzleBenchmark
    {
        private Puzzle puzzle;
        private OrderBasedStrategy  strategy;

        [GlobalSetup]
        public void Setup()
        {
            var builder = new PuzzleBuilder();
            puzzle = builder.BuildTPuzzle();
            strategy = new OrderBasedStrategy();
        }

        //[Benchmark]
        //public void Rotate()
        //{
        //    RotateAndCheck(puzzle, 6);
        //}

        [Benchmark]
        public void PermutateRec()
        {
            PermutateRec(null, null);
        }

        [Benchmark]
        public void PermutateLoop()
        {
            PermutateLoop(Enumerable.Range(0,9).ToArray());
        }

        //[Benchmark]
        //public void IsSolved()
        //{
        //    puzzle.IsSolved([8, 7, 6, 3, 4, 5, 2, 1, 0]);
        //}

        void PermutateRec(Queue<int>? nums, Stack<int>? res)
        {
            nums ??= new Queue<int>(Enumerable.Range(0, 9).OrderByDescending(t => t));
            res ??= new Stack<int>(nums.Count);

            for (int i = 0; i < nums.Count; i++)
            {
                if (nums.TryDequeue(out int cur))
                {
                    res.Push(cur);
                    PermutateRec(nums, res);
                    res.Pop();
                    nums.Enqueue(cur);
                }
            }
        }

        private void PermutateLoop(int[] range)
        {
            for (int i = 0; i < 9; i++)
            {

            }
        }

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
                }

                puzzle1.GetTile(order[i]).Rotate();
            }
        }
    }
}
