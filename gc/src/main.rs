use std::io::{self, stdin, Read, Result};
use std::iter::Peekable;

#[inline]
fn is_nucleobase(b: u8) -> bool {
    b == b'A' ||
    b == b'C' ||
    b == b'G' ||
    b == b'T'
}

#[inline]
fn is_gc(b: u8) -> bool {
    b == b'G' || b == b'C'
}

#[inline]
fn is_newline(b: u8) -> bool {
    b == b'\n' ||
    b == b'\r'
}

fn skip_newline<I>(bs: &mut Peekable<I>)
    where I: Iterator<Item = u8>
{
    loop {
        if let Some(&b) = bs.peek() {
            if is_newline(b) {
                bs.next();
                continue;
            }
        }
        return;
    }
}

fn next_label<I>(bs: &mut Peekable<I>) -> Result<Vec<u8>>
    where I: Iterator<Item = u8>
{
    skip_newline(bs);
    match bs.next() {
        Some(b'>') => (),
        Some(_) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                                             "Labels should start with '>'")),
        None => return Ok(vec![]),
    }

    let mut label = Vec::new();
    for b in bs {
        if is_newline(b) {
            return Ok(label);
        }
        label.push(b);
    }
    Ok(label)
}

fn next_gc_content<I>(bs: &mut Peekable<I>) -> Result<f32>
    where I: Iterator<Item = u8>
{
    let mut total: usize = 0;
    let mut gc: usize = 0;
    loop {
        if let Some(&b'>') = bs.peek() {
            break;
        }

        match bs.next() {
            Some(b) => {
                if is_nucleobase(b) {
                    total += 1;
                    if is_gc(b) {
                        gc += 1;
                    }
                }
            }
            None => break,
        }
    }
    Ok(gc as f32 / total as f32)
}

fn main_io() -> Result<()> {
    let stdin = stdin();
    let mut bs = stdin.lock()
                      .bytes()
                      .map(|b| b.unwrap())
                      .peekable();

    let mut best_label = Vec::new();
    let mut best_gc_content = 0.0_f32;
    loop {
        let label = next_label(&mut bs)?;
        if label.is_empty() {
            println!("{}\n{:.06}\n", String::from_utf8_lossy(&best_label),
                best_gc_content * 100.0_f32);
            break;
        }

        let gc_content = next_gc_content(&mut bs)?;
        if gc_content > best_gc_content {
            best_gc_content = gc_content;
            best_label = label;
        }
    }
    Ok(())
}

fn main() {
    main_io().unwrap();
}
