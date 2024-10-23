// See https://aka.ms/new-console-template for more information

using System.Diagnostics;
using BenchmarkDotNet.Running;
using VerflixtPuzzle.Cmd;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;

var builder = new PuzzleBuilder();
var puzzle = builder.BuildTPuzzle();

var vis = new PuzzleConsoleVisualizer();
vis.Visualize(puzzle, [1, 0, 2, 3, 4, 5, 6, 7, 8]);

Console.WriteLine("Let's solve the puzzle: ");
Console.WriteLine();

//BenchmarkRunner.Run<PuzzleBenchmark>();

var oneFindWatch = Stopwatch.StartNew();
var completeWatch = Stopwatch.StartNew();

int positionsCount = 0;

puzzle.Permutate((puzzle1, order) =>
{
    vis.Visualize(puzzle1, order);
    Console.WriteLine(string.Join(",", order));
    Console.WriteLine($"Time: {oneFindWatch.Elapsed}");
    oneFindWatch.Restart();
    Console.WriteLine();
    positionsCount++;
});

//puzzle.RotateAndSolveCross([0, 1, 2, 3, 4, 5, 6, 7, 8], [], 0, (puzzle1, order) =>
//{
//    vis.Visualize(puzzle1, order);
//    Console.WriteLine(string.Join(",", order));
//    Console.WriteLine();
//});

Console.WriteLine($"Search completed. Found: {positionsCount} positions.");
Console.WriteLine($"Time: {completeWatch.Elapsed}");