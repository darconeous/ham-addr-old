const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-\\^";

fn char_to_index(c: char) -> Result<u64, String> {
    CHARS.find(c).ok_or(format!("bad char '{}'", c)).map(|k| k as u64 + 1)
}

fn chunk_to_hex(chunk: &[char]) -> String {
    assert!(chunk.len() <= 3);
    let (quad, _) = chunk
        .into_iter()
        .map(|c| char_to_index(*c).unwrap())
        .fold((0, 40 * 40), |(sum, m), c| (sum + m * c, m / 40));
    format!("{:04X}", quad)
}

fn callsign_to_ham_addr(callsign: &str) -> String {
    callsign
        .to_uppercase()
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| chunk_to_hex(c))
        .collect::<Vec<_>>()
        .join(":")
}

fn index_to_char(k: usize) -> Result<Option<char>, String> {
    if k == 0 {
        return Ok(None);
    }
    CHARS.chars().nth(k - 1).ok_or(format!("bad index {}", k)).map(|c| Some(c))
}

fn hex_to_chunk(hex: &str) -> Result<String, String> {
    assert!(hex.len() <= 4);
    let hex = format!("{:0<4}", hex);
    let hex = u16::from_str_radix(hex.as_str(), 16).map_err(|_| "Bad hex")?;
    let cs = [
        index_to_char((usize::from(hex) / 1600) % 40)?,
        index_to_char((usize::from(hex) / 40) % 40)?,
        index_to_char((usize::from(hex) / 1) % 40)?,
    ];
    Ok(cs.iter().flatten().collect())
}

#[allow(dead_code)]
fn ham_addr_to_callsign(ham_addr: &str) -> String {
    ham_addr.split(':').map(|h| hex_to_chunk(h).unwrap()).collect::<Vec<_>>().join("")
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    for arg in &args[1..] {
        println!("call: {} -> hamaddr: {}", arg, callsign_to_ham_addr(arg));
    }
}
