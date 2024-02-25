
#[test]
fn grep_main() {
    //string literals stored in pre-allocated read-only memory
    let search = "picture";
    let quote = "\
        Every face, every shop, bedroom window, public - house,
        and dark square is picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";

    //let mut line_num: usize = 1;
    for (i, line) in quote.lines().enumerate() {
        //println!("--->{}", line);
        if line.contains(search) {
            println!("{}: {}", i, line);
        }
        //line_num += 1;
    }
}

