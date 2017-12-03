// [<AutoOpen>]
// module Rosalind

open System;

let lineSeq =
    fun _ -> Console.ReadLine()
    |>  Seq.initInfinite
    |>  Seq.takeWhile ((<>) null)

let dnaSeq (lineSeq: seq<String>) =
    let mutable tag = ""
    let mutable dna = "" // Yes, DNA is mutable
    seq {
        for line in lineSeq do
            if line.StartsWith(">") then
                if dna <> "" then
                    yield (tag, dna)
                    dna <- ""
                tag <- line.[1..]
            else
                dna <- dna + line
        yield (tag, dna)
    }

let buildGraph (dnaSeq: seq<String * String>) =
    let mutable suffixMap = Map.empty
    let prefixList =
        dnaSeq
        |> Seq.map (fun (tag, dna) ->
                        let prefix = dna.[dna.Length - 3..]
                        suffixMap <-
                            match suffixMap.TryFind(prefix) with
                            | Some(tagList) -> suffixMap.Add(prefix, tag :: tagList)
                            | None -> suffixMap.Add(prefix, [tag])
                        (tag, dna.[..3 - 1]))
        |> Seq.toList

    [
        for (toTag, prefix) in prefixList do
            match suffixMap.TryFind(prefix) with
            | Some(tagList) ->
                for fromTag in tagList do
                    if toTag <> fromTag then yield (fromTag, toTag)
            | None -> ()
    ]

let show (tagPairs: (String * String) list) =
    [for (fromTag, toTag) in tagPairs ->
        String.Format("{0} {1}", fromTag, toTag)]

lineSeq
|> dnaSeq
|> buildGraph
|> show
|> List.map (printfn "%s")
