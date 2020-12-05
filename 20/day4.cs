using System;
using System.Linq;
using static System.IO.File;
using static System.Console;
using System.Text.RegularExpressions;
using System.Threading;

var required = new [] {"byr","iyr","eyr","hgt","hcl","ecl","pid"};
var eyecol = new [] {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};

bool numInRange(string y, int min, int max) => int.TryParse(y, out int yr) && yr >= min && yr <= max;
bool heightValid(string h) 
    => (h.Substring(3) == "cm" && numInRange(h.Substring(0,3), 150, 193)) 
    || (h.Substring(2) == "in" && numInRange(h.Substring(0,2), 59, 76));

var pps = ReadAllText(args[0]).Split("\n\n").Select(pp 
    => pp.Split(' ', '\n').Select(d => d.Split(':')).ToDictionary(kv => kv.First(), kv => kv.Last()));
var hasKeys = pps.Where(pp => required.All(v => pp.ContainsKey(v)));
var areValid = hasKeys.Where(pp 
    => numInRange(pp["byr"], 1920, 2002) 
    && numInRange(pp["iyr"], 2010, 2020) 
    && numInRange(pp["eyr"], 2020, 2030) 
    && heightValid(pp["hgt"]) 
    && Regex.IsMatch(pp["hcl"], @"#[0-9|a-f]{6}") 
    && eyecol.Contains(pp["ecl"]) 
    && Regex.IsMatch(pp["pid"], @"^[0-9]{9}$")
);

WriteLine($"Silver: {hasKeys.Count()}");
WriteLine($"Gold: {areValid.Count()}");