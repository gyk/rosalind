namespace Rosalind

open System

module Subs =
  let private find (dna: string) (motif: string) =
    let rec go (from: int) acc =
      let index = dna.IndexOf(motif, from)
      if index >= 0 then
        let index' = index + 1
        go index' (index'::acc)
      else
        List.rev acc
    go 0 []

  let subs () =
    let dna = Console.ReadLine()
    let motif = Console.ReadLine()
    let indices = find dna motif
    indices |> Seq.iter (printf "%d ")
    printf "\b\n"
