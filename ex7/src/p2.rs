use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandKind,
    order: i32,
}

fn card_nr(card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        c if c.is_ascii_digit() => c.to_digit(10).unwrap_or(0) as i32,
        _ => 0,
    }
}

fn hand_parse(hand: &str) -> Hand {
    let mut card_order = 0;
    let mut card_presence = HashMap::new();
    for c in "23456789AJQKT".chars() {
        card_presence.insert(c, 0);
    }

    //dbg!(hand);

    for c in hand.chars() {
        card_order = card_order * 15 + card_nr(c);
        card_presence.get_mut(&c).map(|count| {
            *count += 1;
            Some(())
        });
    }
    
    let wildcards = card_presence.get(&'J').unwrap_or(&0).clone();

    let mut max_card_count = 0;
    let mut max_card_letter = ' ';
    for c in "23456789AQKT".chars() {
        card_presence.get(&c).map(|count| {
            if max_card_count < *count {
                max_card_count = *count;
                max_card_letter = c;
            }
            Some(())
        });
    }

    if max_card_letter != ' ' {
        card_presence.get_mut(&max_card_letter).map(|count| {
            *count += wildcards;
            Some(())
        });

        card_presence.get_mut(&'J').map(|count| {
            *count = 0;
            Some(())
        });
    }
    
    let mut card_appearances = card_presence.values().filter(|v| **v > 0).collect::<Vec<_>>();
    card_appearances.sort_by(|a, b| b.cmp(a));

    let hand_r = if card_appearances[0] >= &5 {
        HandKind::FiveOfAKind
    } else if card_appearances[0] >= &4 {
        HandKind::FourOfAKind
    } else if card_appearances[0] >= &3 {
        if card_appearances[1] >= &2 {
            HandKind::FullHouse
        } else {
            HandKind::ThreeOfAKind
        }
    } else if card_appearances[0] >= &2 {
        if card_appearances[1] >= &2 {
            HandKind::TwoPair
        } else {
            HandKind::OnePair
        }
    } else {
        HandKind::HighCard
    };
    
    Hand {
        kind: hand_r,
        order: card_order
    }
}

fn main() {
    let mut data = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let fields = line
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            (hand_parse(fields[0]), fields[1].parse::<i32>().unwrap_or(0))
        })
        .collect::<Vec<_>>();
    data.sort_by_key(|a| a.0);
    
    let mut result = 0;
    for i in 0..data.len() {
        result += (i as i32 + 1) * data[i].1;
    }

    println!("{result}");
}
