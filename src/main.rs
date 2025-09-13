#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}


fn main() {

    let suits = ["Heart","Diamond","Spade"];
    let values = ["Ace", "One", "Two", "Three"];

    let mut cards = vec![];

    for suit in suits{
        for value in values{
            let card = format!("{} of {}", suit, value);

            cards.push(card);
        }
    }   

    let deck:Deck = Deck{ cards };
    println!("Here is your deck: {:#?}", deck);
}


