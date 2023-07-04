fn main() {
    let day_orders = [ 
        "first",    
        "second",   
        "third",    
        "fourth",   
        "fifth",    
        "sixth",    
        "seventh",  
        "eight",    
        "ninth",    
        "tenth",    
        "eleventh", 
        "twelfth"   
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    let mut i: usize = 0;
    for element in day_orders {
        println!("On the {element} day of Christmas my true love sent me");
        let mut j = i;
        loop {
            println!("and {}",gifts[j]);
            if j == 0 {break;}
            j -= 1;
        }
        i += 1;
        println!();
    }
    
}
