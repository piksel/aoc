
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text.RegularExpressions;
using static System.Console;

var re = new Regex(@"(.+) bags contain (?:(no other|([0-9]+) (\w+ \w+)) bags?(?:, |\.))+");
var map = new Dictionary<string, List<(int Amount, string Color)>>();

foreach (var line in File.ReadLines(args[0]))
{
    var match = re.Match(line);
    var source = match.Groups[1].Value;
    var targets = match.Groups[3].Captures.Zip(match.Groups[4].Captures)
        .Select(c => (int.Parse(c.First.Value), c.Second.Value));

    if(!map.ContainsKey(source))
    {
        map[source] = new List<(int, string)>();
    }
    map[source].AddRange(targets);
}

bool containsGold(string key) => map[key].Any(v => v.Color == "shiny gold" || containsGold(v.Color));
int bagCount(string key) => 1 + map[key].Sum(x => x.Amount * bagCount(x.Color));

WriteLine($"Silver: {map.Count(kv => containsGold(kv.Key))}");
WriteLine($"Gold: {bagCount("shiny gold") - 1}");