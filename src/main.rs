fn main() {
    println!("Hello, world!"); // standard line-ending comment
    /*
    	block comment - which isn't really recommended
    */
    
    do_shit();
    do_more();
    do_other();

}

/// document this shit
fn do_shit() {
}

/**
more documentation?
*/
fn do_more() {
}

/*
no documentation - still ends up in docs, just no text
*/
fn do_other() {

}