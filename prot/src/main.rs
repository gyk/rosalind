use std::io::{self, stdin, BufRead, Result};

fn encode_as_protein(rna: &str) -> Result<String> {
    let rna_len = rna.len();
    assert_eq!(rna_len % 3, 0);

    let protein_len = rna_len / 3 - 1;
    let mut protein = String::with_capacity(protein_len);
    let mut i = 0;
    while i < rna_len {
        match translate(&rna[i .. i + 3])? {
            Some(amino_acid) => protein.push(amino_acid),
            None => {
                if i + 3 == rna_len {
                    return Ok(protein);
                } else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                        "Early stop codon"));
                }
            }
        }
        i += 3;
    }
    return Err(io::Error::new(io::ErrorKind::InvalidData,
        "The RNA does not end with stop codon"));
}

fn translate(codon: &str) -> Result<Option<char>> {
    assert_eq!(codon.len(), 3);
    let aa = match codon {
        "UUU" => Some('F'),
        "UUC" => Some('F'),
        "UUA" => Some('L'),
        "UUG" => Some('L'),
        "UCU" => Some('S'),
        "UCC" => Some('S'),
        "UCA" => Some('S'),
        "UCG" => Some('S'),
        "UAU" => Some('Y'),
        "UAC" => Some('Y'),
        "UAA" => None,
        "UAG" => None,
        "UGU" => Some('C'),
        "UGC" => Some('C'),
        "UGA" => None,
        "UGG" => Some('W'),
        "CUU" => Some('L'),
        "CUC" => Some('L'),
        "CUA" => Some('L'),
        "CUG" => Some('L'),
        "CCU" => Some('P'),
        "CCC" => Some('P'),
        "CCA" => Some('P'),
        "CCG" => Some('P'),
        "CAU" => Some('H'),
        "CAC" => Some('H'),
        "CAA" => Some('Q'),
        "CAG" => Some('Q'),
        "CGU" => Some('R'),
        "CGC" => Some('R'),
        "CGA" => Some('R'),
        "CGG" => Some('R'),
        "AUU" => Some('I'),
        "AUC" => Some('I'),
        "AUA" => Some('I'),
        "AUG" => Some('M'),
        "ACU" => Some('T'),
        "ACC" => Some('T'),
        "ACA" => Some('T'),
        "ACG" => Some('T'),
        "AAU" => Some('N'),
        "AAC" => Some('N'),
        "AAA" => Some('K'),
        "AAG" => Some('K'),
        "AGU" => Some('S'),
        "AGC" => Some('S'),
        "AGA" => Some('R'),
        "AGG" => Some('R'),
        "GUU" => Some('V'),
        "GUC" => Some('V'),
        "GUA" => Some('V'),
        "GUG" => Some('V'),
        "GCU" => Some('A'),
        "GCC" => Some('A'),
        "GCA" => Some('A'),
        "GCG" => Some('A'),
        "GAU" => Some('D'),
        "GAC" => Some('D'),
        "GAA" => Some('E'),
        "GAG" => Some('E'),
        "GGU" => Some('G'),
        "GGC" => Some('G'),
        "GGA" => Some('G'),
        "GGG" => Some('G'),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid codon")),
    };
    Ok(aa)
}


fn main_io() -> Result<()> {
    let mut rna = String::with_capacity(10000);
    {
        let stdin = stdin();
        stdin.lock().read_line(&mut rna)?;
        rna.pop(); // removes newline
    }

    println!("{}", encode_as_protein(&rna)?);
    Ok(())
}

fn main() {
    main_io().unwrap();
}
