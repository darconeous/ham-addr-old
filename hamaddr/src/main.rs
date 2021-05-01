fn hamindex(c: char) -> u64 {
    const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-\\^";
    CHARS.find(c).unwrap_or_default() as u64 + 1
}

fn triple_to_quad(chunk: &[char]) -> String {
    assert!(chunk.len() <= 3);
    let (quad, _) = chunk
        .into_iter()
        .map(|c| hamindex(*c))
        .fold((0, 40 * 40), |(sum, m), c| (sum + m * c, m / 40));
    format!("{:04X}", quad)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    for arg in &args[1..] {
        let call = arg.to_uppercase();
        let hamaddr = call
            .chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|c| triple_to_quad(c))
            .collect::<Vec<_>>()
            .join(":");
        println!("call: {} -> hamaddr: {}", call, hamaddr);
    }
}
