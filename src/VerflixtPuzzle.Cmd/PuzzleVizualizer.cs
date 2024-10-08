using Pastel;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.Puzzle;
using VerflixtPuzzle.Model.TSide;

namespace VerflixtPuzzle.Cmd
{
    public class PuzzleConsoleVisualizer
    {
        public void Visualize(Puzzle puzzle, int[] tilesOrder)
        {
            var initialCursorPosition = Console.GetCursorPosition();

            VisualizeTileLine1(puzzle.GetTile(tilesOrder[0]), puzzle.GetTile(tilesOrder[1]), puzzle.GetTile(tilesOrder[2]));
            Console.WriteLine();
            VisualizeTileLine2(puzzle.GetTile(tilesOrder[0]), puzzle.GetTile(tilesOrder[1]), puzzle.GetTile(tilesOrder[2]));
            Console.WriteLine();
            VisualizeTileLine3(puzzle.GetTile(tilesOrder[0]), puzzle.GetTile(tilesOrder[1]), puzzle.GetTile(tilesOrder[2]));

            Console.WriteLine();

            VisualizeTileLine1(puzzle.GetTile(tilesOrder[3]), puzzle.GetTile(tilesOrder[4]), puzzle.GetTile(tilesOrder[5]));
            Console.WriteLine();
            VisualizeTileLine2(puzzle.GetTile(tilesOrder[3]), puzzle.GetTile(tilesOrder[4]), puzzle.GetTile(tilesOrder[5]));
            Console.WriteLine();
            VisualizeTileLine3(puzzle.GetTile(tilesOrder[3]), puzzle.GetTile(tilesOrder[4]), puzzle.GetTile(tilesOrder[5]));

            Console.WriteLine();

            VisualizeTileLine1(puzzle.GetTile(tilesOrder[6]), puzzle.GetTile(tilesOrder[7]), puzzle.GetTile(tilesOrder[8]));
            Console.WriteLine();
            VisualizeTileLine2(puzzle.GetTile(tilesOrder[6]), puzzle.GetTile(tilesOrder[7]), puzzle.GetTile(tilesOrder[8]));
            Console.WriteLine();
            VisualizeTileLine3(puzzle.GetTile(tilesOrder[6]), puzzle.GetTile(tilesOrder[7]), puzzle.GetTile(tilesOrder[8]));

            Console.WriteLine();
        }

        private void VisualizeTileLine1(SquareTile t1, SquareTile t2, SquareTile t3)
        {
            Console.Write("┌─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t1.Up);
            Console.Write("─┐".Pastel(ConsoleColor.Gray));

            Console.Write("┌─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t2.Up);
            Console.Write("─┐".Pastel(ConsoleColor.Gray));

            Console.Write("┌─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t3.Up);
            Console.Write("─┐".Pastel(ConsoleColor.Gray));
        }

        private void VisualizeTileLine2(SquareTile t1, SquareTile t2, SquareTile t3)
        {
            VisualizeSide(t1.Left);
            Console.Write($" {t1.Position} ");
            VisualizeSide(t1.Right);

            VisualizeSide(t2.Left);
            Console.Write($" {t2.Position} ");
            VisualizeSide(t2.Right);

            VisualizeSide(t3.Left);
            Console.Write($" {t3.Position} ");
            VisualizeSide(t3.Right);
        }

        private void VisualizeTileLine3(SquareTile t1, SquareTile t2, SquareTile t3)
        {
            Console.Write("└─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t1.Down);
            Console.Write("─┘".Pastel(ConsoleColor.Gray));

            Console.Write("└─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t2.Down);
            Console.Write("─┘".Pastel(ConsoleColor.Gray));

            Console.Write("└─".Pastel(ConsoleColor.Gray));
            VisualizeSide(t3.Down);
            Console.Write("─┘".Pastel(ConsoleColor.Gray));
        }

        private void VisualizeSide(ISide side)
        {
            if (side is TortoiseSide tortoiseSide)
                Console.Write(tortoiseSide.Body == Shape.Head ? "H".Pastel(tortoiseSide.Color) : "T".Pastel(tortoiseSide.Color));
        }
    }
}
