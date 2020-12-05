using System.IO;
using System.Collections.Immutable;
using System.Linq;
using static System.Linq.Enumerable;
using static ConLib.PrettyConsole;
    
var seats = new bool[0b1111111111];

foreach (var line in File.ReadLines("input.txt"))
    seats[line.Select(s => "BR".Contains(s) ? 1 : 0).Aggregate(0, (a, b) => b | a << 1)] = true;

var taken = seats.Select((s, i) => s ? i : -1).Where(i => i >= 0).ToImmutableArray();
var max = taken.Max();
var min = taken.Min();

var seat = Range(min, max - min).Single(s => !taken.Contains(s));

WriteLine($"Silver: {max}");
WriteLine($"Gold: {seat}");
