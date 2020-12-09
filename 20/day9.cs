using System;
using System.IO;
using System.Linq;
using static System.Console;

const int prevCount = 25;
var input = File.ReadAllLines(args[0]).Select(line => long.Parse(line));

var naughty = input.Skip(prevCount).Where(
    (n, i) =>
    {
        var prevs = input
            .Skip(i)
            .Take(prevCount);
        return !prevs
            .Any(a => prevs.Any(b => a != b && a + b == n));
    }).First();

WriteLine($"Silver: {naughty}");

var inputLen = input.Count();
var allPos = Enumerable.Range(0, inputLen);

var weakness = allPos.SelectMany(a =>
    allPos.Skip(a).Select(b => input.Skip(a).Take(b-a)).Where(nums => nums.Sum() == naughty)
        .Select(nums => (nums.Min() + nums.Max()))).First();

WriteLine($"Gold: {weakness}");