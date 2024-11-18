use rand::prelude::SliceRandom;

pub fn fortune() {
    let fortunes = vec![
        "The early bird gets the worm, but the second mouse gets the cheese.",
        "You are not illiterate if you can read this fortune.",
        "A witty saying proves nothing.",
        "Today is a good day to code!",
        "In the middle of difficulty lies opportunity.",
        "A journey of a thousand lines of code begins with a single compile.",
    ];
    let fortune = fortunes.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", fortune);
}
