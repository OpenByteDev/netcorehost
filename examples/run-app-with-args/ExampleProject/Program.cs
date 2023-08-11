using System;

public static class Program {
    public static void Main(string[] args) {
        Console.WriteLine($"args[{args.Length}] = [{string.Join(", ", args)}]");
    }
}
