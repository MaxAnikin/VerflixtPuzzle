// See https://aka.ms/new-console-template for more information
using BenchmarkDotNet.Running;
using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzleSimple();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

//BenchmarkRunner.Run<PuzzleBenchmark>();

puzzle.RotateAndSolve([0, 1, 2, 3, 4, 5, 6, 7, 8], [], 0, (puzzle1, order) =>
{
    vis.Visualize(puzzle1, order);
    Console.WriteLine(string.Join(",", order));
    Console.WriteLine();
});

Console.WriteLine("Search completed.");