use std::collections::HashMap;

use candle_core::Result;
use llms_from_scratch_rs::listings::ch02::sample_create_vocab;
use llms_from_scratch_rs::listings::ch02::SimpleTokenizerV2;

fn main() -> Result<()> {
    // Get the vocabulary and unwrap the Result
    let string_vocab = sample_create_vocab()?;
    println!("Length: {}", string_vocab.len());
    let vocab: HashMap<&str, i32> = string_vocab.iter().map(|(s, i)| (s.as_str(), *i)).collect();
    let tokenizer = SimpleTokenizerV2::from_vocab(vocab);

    let text =
        "It's the' last' ' he painted, you know,\"Mrs. Gisburn said with pardonable pride.......";
    let ids = tokenizer.encode(text);
    let text = tokenizer.decode(&ids);

    for id in ids {
        println!("{}", id);
    }
    println!("{}", text);

    // // Create a vector from the vocabulary pairs for sorting
    // let mut entries: Vec<_> = string_vocab.into_iter().collect();

    // // Sort by index value
    // entries.sort_by_key(|(_, idx)| *idx);

    // // Define how many entries to display
    // let n: usize = 1; // You can change this to show more or fewer entries

    // // Print the entries
    // println!("Prime {} parole del vocabolario:", n);
    // for (token, idx) in entries.into_iter().take(n) {
    //     println!("('{}', {})", token, idx);
    // }

    Ok(())
}
