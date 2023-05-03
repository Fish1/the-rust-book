use chapter17::AveragedCollection;


fn main() {
    println!("Hello, world!");
    
    let mut avgc = AveragedCollection::new();
    avgc.add(1);
    avgc.add(9);
    avgc.add(1);
    avgc.add(3);

    println!("avgc average = {}", avgc.average());

}
