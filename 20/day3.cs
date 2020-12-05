using System.Linq;
using static System.IO.File;
using static System.Console;

int GetTreeProd(string[] map, params (int x, int y)[] strats)
    => strats.Aggregate(1, (a, t) => a * Enumerable.Range(0, map.Length / t.y).Count(i => map[t.y * i][t.x * i % map[0].Length] == '#'));

var map = ReadAllLines(args[0]);
WriteLine($"Silver: {GetTreeProd(map, (3, 1))}");
WriteLine($"Gold: {GetTreeProd(map, (1,1), (3,1), (5,1), (7,1), (1,2))}");