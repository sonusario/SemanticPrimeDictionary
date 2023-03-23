use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut dictionary = Dictionary::new();

    // Defining lie using only the base table from https://en.wikipedia.org/wiki/Semantic_primes
    dictionary.add_word(Word::new(
        "lie",
        dictionary.parse_string("something someone say when say something not true because want someone else think say is true")
    ));

    dictionary.add_word(Word::new("what", dictionary.parse_string("something")));
    dictionary.add_word(Word::new("person", dictionary.parse_string("someone")));
    dictionary.add_word(Word::new("does", dictionary.parse_string("now do")));
    dictionary.add_word(Word::new("said", dictionary.parse_string("before say")));
    dictionary.add_word(Word::new("he", dictionary.parse_string("said person")));
    dictionary.add_word(Word::new("says", dictionary.parse_string("now say")));
    dictionary.add_word(Word::new("wants", dictionary.parse_string("now want")));
    dictionary.add_word(Word::new("to", dictionary.parse_string("be")));
    dictionary.add_word(Word::new("it", dictionary.parse_string("said thing")));

    // Defining lie using more natural language (as seen in the wiki page) build from the above definitions
    let new_def = dictionary.parse_string(
        "what a person does when he says something not true because he wants someone to think it true"
    );

    dictionary.words.get_mut("lie").unwrap().add_def(new_def);

    dictionary.lookup("lie").print_def();
}

#[derive(Clone, Debug)]
struct Word {
    name: String,
    definitions: Vec<Vec<Word>>,
    is_prime: bool,
}

impl Word {
    fn new(name: &str, def: Vec<Word>) -> Word {
        Word {
            name: String::from(name),
            definitions: vec![def],
            is_prime: false,
        }
    }

    fn prime(name: &str) -> (String, Word) {
        (
            String::from(name),
            Word {
                name: String::from(name),
                definitions: Vec::with_capacity(0),
                is_prime: true,
            },
        )
    }

    fn add_def(&mut self, def: Vec<Word>) {
        if !self.is_prime {
            self.definitions.push(def)
        }
    }

    fn print_def(&self) {
        println!(
            "{} ; is prime? {} ; is defined by:",
            self.name, self.is_prime
        );
        for def in &self.definitions {
            print!("\t=> ");
            for word in def {
                print!("{} ", word.name);
            }
            println!();
        }
    }
}
struct Dictionary {
    words: HashMap<String, Word>,
}

impl Dictionary {
    fn add_word(&mut self, word: Word) {
        self.words.insert(word.name.to_string(), word);
    }

    fn lookup(&self, name: &str) -> Word {
        self.words.get(name).unwrap().clone()
    }

    fn parse_string(&self, phrase: &str) -> Vec<Word> {
        phrase
            .split(' ')
            .map(|x: &str| self.lookup(x))
            .collect::<Vec<Word>>()
    }

    fn new() -> Dictionary {
        Dictionary {
            words: HashMap::from([
                Word::prime("I"),
                Word::prime("you"),
                Word::prime("someone"),
                Word::prime("people"),
                Word::prime("something"),
                Word::prime("thing"),
                Word::prime("kind"),
                Word::prime("part"),
                Word::prime("this"),
                Word::prime("the"),
                Word::prime("same"),
                Word::prime("a"),
                Word::prime("other"),
                Word::prime("else"),
                Word::prime("another"),
                Word::prime("one"),
                Word::prime("two"),
                Word::prime("some"),
                Word::prime("all"),
                Word::prime("much"),
                Word::prime("many"),
                Word::prime("little"),
                Word::prime("few"),
                Word::prime("good"),
                Word::prime("bad"),
                Word::prime("big"),
                Word::prime("small"),
                Word::prime("think"),
                Word::prime("know"),
                Word::prime("want"),
                Word::prime("feel"),
                Word::prime("see"),
                Word::prime("hear"),
                Word::prime("say"),
                Word::prime("word"),
                Word::prime("words"),
                Word::prime("true"),
                Word::prime("do"),
                Word::prime("happen"),
                Word::prime("move"),
                Word::prime("be"),
                Word::prime("there"),
                Word::prime("is"),
                Word::prime("mine"),
                Word::prime("live"),
                Word::prime("die"),
                Word::prime("when"),
                Word::prime("time"),
                Word::prime("now"),
                Word::prime("before"),
                Word::prime("after"),
                Word::prime("long"),
                Word::prime("short"),
                Word::prime("moment"),
                Word::prime("where"),
                Word::prime("place"),
                Word::prime("here"),
                Word::prime("above"),
                Word::prime("below"),
                Word::prime("far"),
                Word::prime("near"),
                Word::prime("side"),
                Word::prime("inside"),
                Word::prime("touch"),
                Word::prime("not"),
                Word::prime("maybe"),
                Word::prime("can"),
                Word::prime("because"),
                Word::prime("if"),
                Word::prime("very"),
                Word::prime("more"),
                Word::prime("like"),
                Word::prime("as"),
                Word::prime("way"),
            ]),
        }
    }
}
