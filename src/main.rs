use rand::Rng;
use std::io::{self};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    const PAUSE: f32 = 0.5;

    let rows = vec![
        "         ##", // Index 0 has no {}.
        "        #{}-{}#",
        "       #{}---{}#",
        "      #{}-----{}#",
        "     #{}------{}#",
        "    #{}------{}#",
        "    #{}-----{}#",
        "     #{}---{}#",
        "     #{}-{}#",
        "      ##", // Index 9 has no {}.
        "     #{}-{}#",
        "     #{}---{}#",
        "    #{}-----{}#",
        "    #{}------{}#",
        "     #{}------{}#",
        "      #{}-----{}#",
        "       #{}---{}#",
        "        #{}-{}#",
    ];

    println!("DNA Animation, by Al Sweigart");
    println!("Press Ctrl-C to quit...");
    thread::sleep(Duration::from_secs(2));

    let mut row_index = 0;
    let mut rng = rand::thread_rng();

    loop {
        row_index += 1;
        if row_index == rows.len() {
            row_index = 0;
        }

        if row_index == 0 || row_index == 9 {
            println!("{}", rows[row_index]);
            thread::sleep(Duration::from_secs_f32(PAUSE));
            continue;
        }

        let (left_nucleotide, right_nucleotide) = match rng.gen_range(1..=4) {
            1 => ('A', 'T'),
            2 => ('T', 'A'),
            3 => ('C', 'G'),
            4 => ('G', 'C'),
            _ => unreachable!(),
        };

        let row_template = rows[row_index];
        let row_with_nucleotides = row_template
            .replacen("{}", &left_nucleotide.to_string(), 1)
            .replacen("{}", &right_nucleotide.to_string(), 1);
        println!("{}", row_with_nucleotides);

        // Pausa
        thread::sleep(Duration::from_secs_f32(PAUSE));
    }
}
