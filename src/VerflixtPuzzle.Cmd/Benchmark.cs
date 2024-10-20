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
        //public void Permutate()
        //{
        //    puzzle.Permutate((puzzle1, order) => {});
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

        //[Benchmark]
        //public void GeneratePermutationsSpan()
        //{
        //    GeneratePermutationsSpan(new Span<int>([0, 1, 2, 3, 4, 5, 6, 7, 8]), 9);
        //}

        //[Benchmark]
        //public void RotateAndCheck()
        //{
        //    var str = new DefaultIsSolvedStrategy([]);
        //    RotateAndCheck(puzzle, str, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        //}

        //[Benchmark]
        //public void RotateAndCheck2()
        //{
        //    RotateAndCheck2(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        //}

        //[Benchmark]
        //public void IsSolved()
        //{
        //    puzzle.IsSolved([8, 7, 6, 3, 4, 5, 2, 1, 0]);
        //}

        [Benchmark]
        public void RotateAndSolve()
        {
            puzzle.RotateAndSolve([0, 1, 2, 3, 4, 5, 6, 7, 8], [], 0, (puzzle1, order) => { });
        }
    }
}
