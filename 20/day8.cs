using System;
using System.IO;
using System.Linq;
using static System.Console;

var rom = File.ReadAllLines(args[0]).Select(line => line.Split(' ')).Select(p => (Op: p[0], Arg: int.Parse(p[1]))).ToArray();

var ip = 0;
var ra = 0;
var sw = 0;
var pg = rom.ToArray();

while (ip < pg.Length)
{
    var (op, arg) = pg[ip];
    pg[ip] = ("hlt", 0);

    if (ip == sw && op == "nop") { op = "jmp"; }
    else if (ip == sw && op == "jmp") { op = "nop"; }
    
    switch (op)
    {
        case "nop":
            ip++;
            continue;
        case "acc":
            ra += arg;
            ip++;
            continue;
        case "jmp":
            ip += arg;
            continue;
    }
    
    if (sw == 0) WriteLine($"Silver: {ra}");
    
    ip = 0;
    ra = 0;
    pg = rom.ToArray();

    do sw++;
    while (pg[sw].Op != "nop" && pg[sw].Op != "jmp");
}

WriteLine($"Gold: {ra}");