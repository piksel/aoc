using System.IO;
using System.Linq;
using static System.Console;

var answers = File.ReadAllText(args[0]).Split("\n\n")
    .Select(g => g.Split('\n').Where(line => line.Length > 0));

var groupYesAnswers = answers
    .Select(g => string.Concat(string.Concat(g).Distinct()).Length)
    .Sum();

var groupAllYes = answers
    .Select(g => g.SelectMany(p => p.ToArray()).Distinct())
    .Zip(answers).Select(g => 
        g.First
            .Where(a => 
                g.Second.All(ga => ga.Contains(a)))
            .Count())
    .Sum();

WriteLine($"Silver: {groupYesAnswers}");
WriteLine($"Gold: {groupAllYes}");