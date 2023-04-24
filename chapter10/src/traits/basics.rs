use std::fmt::Display;

trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("Default from {}", self.summarize_author())
  }
}

trait Joker {
  fn make_joke(&self) {
    println!("Why did the chicken cross the road???");
  }
}

struct NewsArticle {
  pub headline: String,
  pub author: String,
  pub text: String,
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }

  fn summarize(&self) -> String {
    format!(
      "{}, by {}",
      self.headline,
      self.author
    )
  }
}

struct Tweet {
  pub author: String,
  pub text: String
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("!{}", self.author)
  }

  fn summarize(&self) -> String {
      format!(
        "twitter.com {}, by {}",
        self.text,
        self.author
      )
  }
}

impl Joker for Tweet { }

struct Book {
  pub title: String,
  pub author: String,
  pub isbn: String,
}

impl Summary for Book {
  fn summarize_author(&self) -> String {
    format!("!!{}", self.author)
  }
}

struct Car {
  pub make: String,
  pub model: String
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      println!("Make: {}, Model: {}", self.make, self.model);
      Ok(())
    }
}

fn print_summary<Type: Summary>(summarizable: &Type) {
  println!("SUMMARY FUNCTION: {}", summarizable.summarize());
}

fn print_summary_2(summarizable: &impl Summary) {
  println!("SUMMARY FUNCTION 2: {}", summarizable.summarize());
}

fn do_stuff_1(thing: &(impl Summary + Joker)) {
  thing.make_joke();
}

fn do_stuff_2<Type>(thing: &Type) 
  where Type: Summary + Joker {
  thing.make_joke();
  println!("Summary: {}", thing.summarize());
}

fn create_summarizable() -> impl Summary {
  Tweet {
    author: String::from("testing"),
    text: String::from("this is my epic tweet")
  }
}

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x, y
    }
  }
}

impl<T: PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("x > y");
    } else if self.x < self.y {
      println!("x < y");
    } else {
      println!("x = y");
    }
  }
}

pub fn do_basics() {

  let article = NewsArticle {
    headline: String::from("The world is ending!!!"),
    author: String::from("Bobby Jones"),
    text: String::from("These are the reasons the world is ending... blah blah blah blah")
  };

  let tweet = Tweet {
    author: String::from("Jacob"),
    text: String::from("I love twitter.com!")
  };

  let car = Car {
    make: String::from("Honda"),
    model: String::from("Civic")
  };
  println!("Car: {}", car);

  let book = Book {
    title: String::from("Rust Programming"),
    author: String::from("Jacob"),
    isbn: String::from("1231230932")
  };

  print_summary(&article);
  print_summary(&tweet);
  // we cannot call print_summary with car
  // because car doesn't implement summary
  // print_summary(car);

  // book implements a default summarize function
  print_summary(&book);
  print_summary_2(&book);

  do_stuff_1(&tweet);
  do_stuff_2(&tweet);

  print_summary(&create_summarizable());

  let mypair = Pair::new(3, 2);
  mypair.cmp_display();

  let otherpair = Pair::new(&tweet, &tweet);
  // otherpair doesn't have cmp_display because the Tweet type doesn't implement PartialCmp
  // otherpair.cmp_display();
}