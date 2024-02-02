use std::io;
use std::cmp::Ordering;
use counter::Counter;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Card {
  Jack,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Queen,
  King,
  Ace
}
impl From<u8> for Card {
  fn from(c: u8) -> Card {
    use Card::*;
    match c {
      b'2' => Two,
      b'3' => Three,
      b'4' => Four,
      b'5' => Five,
      b'6' => Six,
      b'7' => Seven,
      b'8' => Eight,
      b'9' => Nine,
      b'T' => Ten,
      b'J' => Jack,
      b'Q' => Queen,
      b'K' => King,
      b'A' => Ace,
      _ => panic!("Card::from passed invalid character {c}"),
    }
  }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Rank {
  High,
  Pair,
  TwoPair,
  Three,
  FullHouse,
  Four,
  Five
}

impl From<&[Card; 5]> for Rank {
  fn from(c: &[Card; 5]) -> Rank {
    let mut c = c.iter().copied().collect::<Counter<Card, usize>>();
    let num_jacks = if c.contains_key(&Card::Jack) {
      let tp = c[&Card::Jack];
      c.remove(&Card::Jack);
      tp
    } else { 0 };
    let mut sorted_values = c.values().copied().collect::<Vec<usize>>();
    sorted_values.sort();
    let last = sorted_values.last_mut();
    match last {
      None => {
        sorted_values = vec!(5);
      }
      Some(last) => {
        *last += num_jacks;
      }
    }
    use Rank::*;
    match sorted_values[..] {
      [1, 1, 1, 1, 1] => High,
      [1, 1, 1, 2] => Pair,
      [1, 2, 2] => TwoPair,
      [1, 1, 3] => Three,
      [2, 3] => FullHouse,
      [1, 4] => Four,
      [5] => Five,
      _ => panic!("There's a bug in the matrix: {sorted_values:?}")
    }
  }
}

#[derive(Eq, Debug)]
struct Hand {
  rank: Rank,
  cards: [Card; 5],
  bid: u64,
}

impl Hand {
  fn new(cards: [Card; 5], bid: u64) -> Self {
    let rank = Rank::from(&cards);
    Hand { rank, cards, bid }
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    (&self.rank, &self.cards).cmp(&(&other.rank, &other.cards))
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.cards == other.cards
  }
}

fn main() {
  let mut hands = Vec::new();
  for line in io::stdin().lines() {
    let line = line.unwrap();
    let [cards, bid]: [&str; 2] = line.split(' ').collect::<Vec<&str>>().try_into().unwrap();
    let cards: [Card; 5] = cards.as_bytes().iter().map(|x| Card::from(*x)).collect::<Vec<_>>()
    .try_into().expect("wrong size iterator");
    let bid = bid.parse::<u64>().unwrap();
    hands.push(Hand::new(cards, bid));
  }
  hands.sort();
  let answer = hands.iter().enumerate().fold(0, |cum, (i, hand)| cum + ((i+1) as u64) * hand.bid);
  println!("{}", answer);
}