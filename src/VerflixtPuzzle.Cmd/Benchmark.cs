using BenchmarkDotNet.Attributes;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;

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
            puzzle = builder.BuildTPuzzleSimple();
        }

        //[Benchmark]
        //public void Rotate()
        //{
        //    RotateAndCheck(puzzle, 6);
        //}

        //[Benchmark]
        //public void PermutateRec()
        //{
        //    PermutateRec(null, null);
        //}

        //[Benchmark]
        //public void PermutateLoop()
        //{
        //    PermutateLoop(Enumerable.Range(0, 9).ToArray());
        //}

        //[Benchmark]
        //public void GeneratePermutations()
        //{
        //    GeneratePermutations([0, 1, 2, 3, 4, 5, 6, 7, 8], 9);
        //}

        [Benchmark]
        public void RotateAndCheck()
        {
            RotateAndCheck(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        }

        //[Benchmark]
        //public void IsSolved()
        //{
        //    puzzle.IsSolved([8, 7, 6, 3, 4, 5, 2, 1, 0]);
        //}

        //[Benchmark]
        //public void RotateAndSolve()
        //{
        //    for (int i = 0; i < 4; i++)
        //    {
        //        puzzle.GetTile(0).Rotate();
        //        puzzle.IsSolved(new DefaultIsSolvedStrategy([0,1,2,3,4,5,6,7,8]));
        //    }
        //}

        void GeneratePermutations(int[] arr, int size)
        {
            if (size == 1)
            {
                return;
            }

            for (int i = 0; i < size; i++)
            {
                GeneratePermutations(arr, size - 1);

                if (size % 2 == 0)
                {
                    Swap(ref arr[i], ref arr[size - 1]);
                }
                else
                {
                    Swap(ref arr[0], ref arr[size - 1]);
                }
            }
        }

        void Swap(ref int a, ref int b)
        {
            (a, b) = (b, a);
        }

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
                    RotateAndCheck(puzzle1, order, i + 1);

                new DefaultIsSolvedStrategy(order).IsSolved(puzzle1, order[i]);

                puzzle1.GetTile(order[i]).Rotate();
            }
        }
    }
}
