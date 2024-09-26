using Pastel;
using VerflixtPuzzle.Model;
using VerflixtPuzzle.Model.TSide;

namespace VerflixtPuzzle.Cmd
{
    public interface IPuzzleVisualizer
    {
        public void Visualize(Puzzle puzzle);
    }

    public class PuzzleConsoleVisualizer : IPuzzleVisualizer
    {

        public void Visualize(Puzzle puzzle)
        {
            var initialCursorPosition = Console.GetCursorPosition();

            for (int i = 0; i < 3; i++)
            {
                VisualizeTile(puzzle.GetTile(i));
                Console.SetCursorPosition(initialCursorPosition.Left + ((i+1) * 7), initialCursorPosition.Top);
            }
            
            Console.SetCursorPosition(initialCursorPosition.Left, initialCursorPosition.Top + 4);


            for (int i = 3; i < 6; i++)
            {
                VisualizeTile(puzzle.GetTile(i));
                Console.SetCursorPosition(initialCursorPosition.Left + ((i-2) * 7), initialCursorPosition.Top+4);
            }

            Console.SetCursorPosition(initialCursorPosition.Left, initialCursorPosition.Top + 8);

            for (int i = 6; i < 9; i++)
            {
                VisualizeTile(puzzle.GetTile(i));
                Console.SetCursorPosition(initialCursorPosition.Left + ((i-5) * 7), initialCursorPosition.Top+8);
            }

            Console.SetCursorPosition(initialCursorPosition.Left, initialCursorPosition.Top+12);
        }

        private void VisualizeTile(Tile tile)
        {
            var initialCursorPosition = Console.GetCursorPosition();

            Console.Write($"┌───┐".Pastel(ConsoleColor.Gray));
            //Console.Write($"│   │".Pastel(ConsoleColor.Gray));
            Console.SetCursorPosition(initialCursorPosition.Left, initialCursorPosition.Top + 2);
            Console.Write($"└───┘".Pastel(ConsoleColor.Gray));



            Console.SetCursorPosition(initialCursorPosition.Left + 2, initialCursorPosition.Top);
            VisualizeSide(tile.Up);

            Console.SetCursorPosition(initialCursorPosition.Left, initialCursorPosition.Top + 1);
            VisualizeSide(tile.Left);

            Console.SetCursorPosition(initialCursorPosition.Left + 4, initialCursorPosition.Top + 1);
            VisualizeSide(tile.Right);

            Console.SetCursorPosition(initialCursorPosition.Left + 2, initialCursorPosition.Top + 2);
            VisualizeSide(tile.Down);
        }

        private void VisualizeSide(Side side)
        {
            if (side is TortoiseSide tortoiseSide)
                Console.Write(tortoiseSide.Body == Shape.Head ? "H".Pastel(tortoiseSide.Color) : "T".Pastel(tortoiseSide.Color));
        }
    }
}
