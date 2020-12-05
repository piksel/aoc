using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Threading;
using System.Threading.Tasks;
using ConLib;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using static ConLib.PrettyConsole;
using  Microsoft.CodeAnalysis.Scripting;

const string sourcePattern = "day*.cs";

var targetDay = args.FirstOrDefault(a => a[0] != '-');
var watch = args.Contains("-w") || args.Contains("--watch");
var assembly = Assembly.GetExecutingAssembly();
var tokens = new Dictionary<string, CancellationTokenSource>();
var runTimeout = TimeSpan.FromSeconds(30);
FileSystemWatcher watcher = null; 

WriteLine($"{"Advent of Code"} {2020}\n");

var sourceRoot = new DirectoryInfo(Environment.CurrentDirectory);
while (!sourceRoot.EnumerateFiles(sourcePattern).Any())
{
    sourceRoot = sourceRoot.Parent ?? throw new DirectoryNotFoundException("Could not find source directory");
}

if (watch)
{
    WriteLine($"{"Watching"} source directory: {sourceRoot.FullName}");
    watcher = new FileSystemWatcher(sourceRoot.FullName, targetDay == "all" ? sourcePattern : $"day{targetDay}.cs");
     watcher.Changed += (_, ea) =>
     {
         var dayNum = ea.Name is {} name ? dayFromSource(name) : null;
         if (dayNum is {} && tokens.TryGetValue(dayNum, out var cts))
         {
             cts.Cancel();
         }
     };
}
else
{
    WriteLine($"Using source directory: {sourceRoot.FullName}");
}

if (targetDay is null)
{
    WriteLine($"Available days:");
}


foreach (var dayFile in sourceRoot.EnumerateFiles(sourcePattern))
{
    var dayNum = dayFromSource(dayFile.Name);
    if (targetDay is null)
    {
        WriteLine($"  dotnet run {dayNum}");
    }
    if (targetDay != "all" && targetDay != dayNum)
    {
        continue;
    }

    CompileAndRun(dayFile.FullName, dayNum);
}

while (watcher is {})
{
    var wcr = watcher.WaitForChanged(WatcherChangeTypes.Changed);
    if(!(wcr.Name is {} name)) continue;
    
    WriteLine($"Watched file {name} {wcr.ChangeType}!");
    var dayNum = dayFromSource(name);

    Task.Run(() => CompileAndRun(Path.Join(sourceRoot.FullName, name), dayNum)).Wait();
}

void CompileAndRun(string dayFile, string dayNum)
{
    tokens[dayNum] = new CancellationTokenSource(runTimeout);
    var ct = tokens[dayNum].Token;

    Script? script = null;
    
    foreach (var attempt in Enumerable.Range(0, 10))
    {
        if(DoTask($"Read Day {dayNum} source{(attempt>0?$" (retry {attempt}":"")}", () =>
        {
            var src = File.ReadAllText(dayFile)
                .Replace("using static System.Console", "using static ConLib.PrettyConsole");
            script = CSharpScript.Create(src, ScriptOptions.Default.WithReferences(
                typeof(Enumerable).Assembly,
                typeof(ConCol).Assembly,
                assembly
            ), typeof(Globals));
            return Task.CompletedTask;
        })) break;
        DoTask("Waiting 1 second", () => Task.Delay(TimeSpan.FromSeconds(1)));
    }

    DoTask($"Compile Day {dayNum}", () => Task.FromResult(script.Compile(ct)));
    
    DoChore($"Run Day {dayNum}", () =>
    {
        if (script.RunAsync(new Globals(new []{ Path.Join(sourceRoot.FullName, "inputs", $"day{dayNum}.txt")}), _ => true, ct).Result.Exception is { } x) throw x;
    });

    if (ct.IsCancellationRequested)
    {
        WriteColor("Cancelled!", ConCol.Yellow);
    }
}

string dayFromSource(string fn) => fn.Substring(3, fn.Length - 6);

public record Globals(string[] args) {}