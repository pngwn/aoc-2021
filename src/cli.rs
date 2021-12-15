use ansi_term::Colour;
use ansi_term::{ANSIString, ANSIStrings};

pub fn run() {
    let tree = make_tree();
    println!("\n");
    for line in tree.iter() {
        println!("{}", ANSIStrings(line));
    }

    println!("=================");
    println!(" ADVENT OF CöööD");
    println!("=================");
}

fn get_color(c: String) -> ANSIString<'static> {
    match c.as_str() {
        "\\" => Colour::RGB(255, 255, 0).paint(c), // yellow
        "/" => Colour::RGB(255, 255, 0).paint(c),  // yellow
        "|" => Colour::RGB(255, 255, 0).paint(c),  // yellow
        "*" => Colour::RGB(255, 255, 0).paint(c),  // yellow
        "-" => Colour::RGB(255, 255, 0).paint(c),  // yellow
        ">" => Colour::RGB(50, 155, 50).paint(c),  // green
        "<" => Colour::RGB(50, 155, 50).paint(c),  // green
        "o" => Colour::RGB(255, 150, 0).paint(c),  // orange
        "@" => Colour::RGB(255, 0, 0).paint(c),    // red
        "0" => Colour::RGB(0, 100, 255).paint(c),  // blue
        _ => Colour::RGB(150, 150, 150).paint(c),  // grey
    }
}

fn make_tree() -> Vec<Vec<ANSIString<'static>>> {
    let tree_def: Vec<&str> = vec![
        "|",
        "\\|/",
        "--*--",
        ">o<",
        ">*<<<",
        ">o>o<@<",
        ">0>>>*<0<",
        ">>o<*<*<<<<",
        ">>*>@>>>o<<<<",
        ">*>@<<<0>*<<<0<",
        ">>0<@<<*<o<<@>0<<",
        "_ __│ │__ _",
    ];

    let mut formatted_tree: Vec<Vec<ANSIString>> = Vec::new();

    for s in &tree_def {
        let mut line: Vec<ANSIString> = Vec::new();
        let _str = pad(17, s);
        let x = _str.split("");
        for s2 in x {
            line.push(get_color(s2.to_owned()));
        }

        formatted_tree.push(line)
    }

    formatted_tree
}

fn pad(max: u32, string: &str) -> String {
    let mut _str = repeat(' ', (max - string.chars().count() as u32) / 2);

    _str.push_str(string);

    _str
}

fn repeat(c: char, times: u32) -> String {
    let mut _str = "".to_owned();
    for _ in 0..times {
        _str.push(c);
    }

    _str
}
