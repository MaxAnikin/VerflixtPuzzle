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

        [Benchmark]
        public void Rotate()
        {
            RotateAndCheck(puzzle, 6);
        }

        //[Benchmark]
        //public void Permutate()
        //{
        //    Permutate([8, 7, 6, 3, 4, 5, 2, 1, 0]);
        //}

        [Benchmark]
        public void IsSolved()
        {
            puzzle.IsSolved([8, 7, 6, 3, 4, 5, 2, 1, 0]);
        }

        private void Permutate(int[] order)
        {
            puzzle.Permutate(order);
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
