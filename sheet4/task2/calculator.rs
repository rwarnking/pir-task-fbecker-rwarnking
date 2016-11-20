fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }
        if let Some(calc) = tokenize(&input) {
            // Debug output
            println!("{:#?}", calc);
            let (tree, _) =  Expr::parse(&calc, 0);
            println!("{:#?}", tree);
            println!("{}", tree.evaluate(0));
        } else {
            println!("Input was invalid! Try again:");
            continue;
        };
    }
}

fn tokenize(line: &str) -> Option<Vec<Token>> {
    let mut ret = Vec::new();
    let mut result = line.split_whitespace();

    while let Some(elem) = result.next() {
        for c in elem.chars() {
            match c {
                '(' => ret.push(Token::Open( '(' )),
                ')' => ret.push(Token::Close( ')' )),
                '+' | '-' | '*' | '/' | '%' => ret.push(Token::Operation(c)),
                _ => if c.is_numeric() {
                    ret.push(Token::Number(c.to_digit(10).unwrap() as i32))
                } else { return None },
            }
        }
    };

    Some(ret)
}

/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

#[derive(Debug)]
enum Expr {
    Leaf(Token),
    Internal {
        children: Vec<Expr>,
        data: Token,
    }
}

impl Expr {

    fn parse(tokens: &[Token], mut index: usize) -> (Self, usize) {
        let mut node = Expr::Internal{ children: vec![], data: Token::Operation('+') };

        while index < tokens.len() {
            match tokens[index] {
                Token::Number(i32) => node.add_leaf_child(tokens[index]),
                Token::Operation(char) => node.data_mut().set_operation(tokens[index]),
                Token::Open(char) => {
                    let (child, new_index) = Expr::parse(tokens, index + 1);
                    index = new_index;
                    node.add_child(child)
                },
                Token::Close(char) => return (node, index),
            }
            index += 1;
        }
        (node, index)
    }

    /// Determines if this tree is just a leaf node.
    fn is_leaf(&self) -> bool {
        match *self {
            Expr::Leaf(Token) => true,
            _ => false,
        }
    }

    /// Returns the value attached to this node.
    fn data(&self) -> Token {
        match *self {
            Expr::Leaf(data) => data,
            Expr::Internal { data, .. } => data,
        }
    }

    fn data_mut(&mut self) -> &mut Token {
        match *self {
            Expr::Leaf(ref mut data) => data,
            Expr::Internal { ref mut data, .. } => data,
        }
    }

    /// Adds another tree as a child of this node.
    fn add_child(&mut self, child: Expr) {
        if let Expr::Leaf(data) = *self {
            *self = Expr::Internal {
                children: vec![],
                data: data,
            };
        }

        match *self {
            Expr::Internal { ref mut children, .. } => {
                children.push(child);
            }
            _ => unreachable!(),
        }
    }

    /// Adds another leaf node with the given value as child of this node.
    fn add_leaf_child(&mut self, child_data: Token) {
        self.add_child(Expr::Leaf(child_data));
    }

    /// If this node is internal, this function returns a slice of all
    /// children.
    fn children(&self) -> Option<&[Expr]> {
        match *self {
            Expr::Internal { ref children, .. } => Some(children),
            _ => None,
        }
    }

    /// If this node is internal, this function returns a mutable slice of all
    /// children.
    fn children_mut(&mut self) -> Option<&mut [Expr]> {
        match *self {
            Expr::Internal { ref mut children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn evaluate(&self, mut result: i32) -> i32 {
        if let Some(next) = self.children() {
            let mut index = 0;
            while index < next.len() {
                match next[index].is_leaf() {
                    true => match next[index+1].is_leaf() {
                        true => return result +
                          Op::operation(self.data().char_data()).apply(next[index].data().number(),
                          next[index + 1].data().number()),
                        false => {
                          result +=
                          Op::operation(self.data().char_data()).apply(next[index].data().number(),
                          next[index + 1].evaluate(result)); index += 1;
                        },
                    },
                    false => match next[index+1].is_leaf() {
                        true => {
                          result +=
                        Op::operation(self.data().char_data()).apply(next[index].evaluate(result),
                          next[index + 1].data().number()); index += 1;
                        },
                        false => result += next[index].evaluate(result),
                    },
                }
                index += 1;
            }
            return result;
        };
        0
    }
}

#[derive(Debug, Copy, Clone)]
struct Op {
    op: char,
}

impl Op {

    pub fn data(&self) -> char {
        self.op
    }

    pub fn operation(c: char) -> Self {
        Op{ op: c }
    }

    pub fn apply(&self, num_one: i32, num_two: i32) -> i32 {
        match self.op {
            '+' => num_one + num_two,
            '-' => num_one - num_two,
            '*' => num_one * num_two,
            '/' => num_one / num_two,
            '%' => num_one % num_two,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Operation(char),
    Open(char),
    Close(char),
    Number(i32),
}

impl Token {

    fn is_operator(&self) -> bool {
        match *self {
            Token::Operation(char) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match *self {
            Token::Number(i32) => true,
            _ => false,
        }
    }

    fn number(&self) -> i32 {
        match *self {
            Token::Number(data) => data,
            _ => 0,
        }
    }

    fn char_data(&self) -> char {
        match *self {
            Token::Number(i32) => ' ',
            Token::Open(data) | Token::Close(data) | Token::Operation(data) => data,
        }
    }

    fn set_operation(&mut self, t: Token) {
        *self = t;
    }

    fn to_string(&self) -> String {
        match *self {
            Token::Number(data) => data.to_string(),
            Token::Open(data) | Token::Close(data) | Token::Operation(data) => data.to_string(),
        }
    }
}
