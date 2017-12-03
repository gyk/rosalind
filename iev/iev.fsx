open System;

let probList = [|
  1.0 - 0.0 * 0.0; // AA-AA
  1.0 - 0.0 * 0.5; // AA-Aa
  1.0 - 0.0 * 1.0; // AA-aa
  1.0 - 0.5 * 0.5; // Aa-Aa
  1.0 - 0.5 * 1.0; // Aa-aa
  1.0 - 1.0 * 1.0; // aa-aa
|];

let numbers = Console.ReadLine().Split()
              |> Seq.filter (fun s -> s <> "")
              |> Seq.map float
let expected = Seq.zip numbers probList
               |> Seq.sumBy (fun (n, p) -> n * p)
               |> (*) 2.0
printfn "%f" expected
