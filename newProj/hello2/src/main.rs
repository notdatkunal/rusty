fn main() {
   greet_world();
}

fn greet_world(){
    let my_string = "\
    this is a
    multiline
    added another line for demo
    Hello, world!
    text
    ";
    
    println!("{}",&my_string);
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
        
    }
    let num = 3;
    if num%2==0 {
        println!{"this is even digit"}
    }
    else{
        println!{"this is odd digit"}
    }
    
}
