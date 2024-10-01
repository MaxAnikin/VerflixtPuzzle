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

        [GlobalSetup]
        public void Setup()
        {
            var builder = new PuzzleBuilder();
            puzzle = builder.BuildTPuzzle();
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

        private void RotateAndCheck(Puzzle puzzle1, int i)
        {
            for (int j = 0; j < 4; j++)
            {
                puzzle1.GetTile(0).Rotate();
            }
        }
    }
}
