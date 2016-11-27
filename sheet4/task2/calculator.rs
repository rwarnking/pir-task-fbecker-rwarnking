// If you wish to exit the program type 'exit'
// However, make sure that 'exit' is not in the same line
// as a computation you wish to execute, because the program
// will quit immediately!
fn main() {

    // Small helper function to test very basic (!) functionality
    // test_tree("3 + ( 6 - 1 )");

    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // If tokenize and parse don't result in an error evaluate the tree
        if test_tree(&input) {
            break;
        }
    }
}

// Makes a tree from the given string and evaluates it if possible
// Returns true if it is supposed to stop, else false
fn test_tree(line: &str) -> bool {
    match tokenize(line) {
        Ok(calc) => {
            match Expr::parse(&calc, 0) {
                Ok((tree, _)) => if let Some(result) = tree.evaluate() {
                    println!("Result: {}", result)
                },
                Err(why) => println!("{}", why),
            };
            false
        },
        Err(why) => match why {
            0 => { println!("Could not read number. Try again:"); false },
            _ => true,
        },
    }
}

// Split input into Tokens
fn tokenize(line: &str) -> Result<Vec<Token>, u8> {
    let mut ret = Vec::new();

    if line == "exit" {
        return Err(1);
    }

    for c in line.chars() {
        match c {
            c if c.is_whitespace() => continue,
            '(' => ret.push(Token::Open),
            ')' => ret.push(Token::Close),
            '+' | '-' | '*' | '/' | '%' => ret.push(Token::Operation{ op: Op{op: c }}),
            _ => if let Some(num) = c.to_digit(10) {
                ret.push(Token::Number(num as i64))
            } else {
                return Err(0)
            },
        }
    }

    Ok(ret)
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

    fn parse(tokens: &[Token], mut index: usize) -> Result<(Self, usize), &str> {
        // node to integrate into the tree
        let mut node = Expr::Leaf(Token::Operation{ op: Op{ op: '+' }});
        // 0 = nothing, 1 = first number, 2 = second number, 3 = operator
        // controls for checking if the input is valid
        let mut current = 0;

        while index < tokens.len() {
            match tokens[index] {
                // If we find a number, and the previous Token was an operator or
                // this is the first Token, add a leaf to the tree
                // If we find a number directly after a number set leaf data
                // to their concatenation
                Token::Number(i32) => match current {
                    0 => { current = 1; node.add_leaf_child(tokens[index])},

                    1 | 2  =>
                    if let Some(children) = node.children_mut() {
                        children[current - 1].data_mut().set_number(tokens[index].data());
                    } else {
                        return Err("Could not change node children. Try again:");
                    },

                    3 => { current = 2; node.add_leaf_child(tokens[index]) },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find an operator and the previous Token was a number change the node data
                Token::Operation { op } => match current {
                    1 => {
                        node.data_mut().set_operation(tokens[index].op_data());
                        current = 3;
                    },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find an opening bracket, and the previous Token was an operator or
                // this is the first Token, add a leaf to the tree through recursion
                Token::Open => match current {
                    0 => {
                        if let Ok((child, new_index)) = Expr::parse(tokens, index + 1) {
                            index = new_index;
                            node.add_child(child);
                            current = 1
                        } else {
                            return Err("Input incorrect (maybe the order was wrong). Try again:");
                        }
                    },
                    3 => {
                        if let Ok((child, new_index)) = Expr::parse(tokens, index + 1) {
                            index = new_index;
                            node.add_child(child);
                            current = 2
                        } else {
                            return Err("Input incorrect (maybe the order was wrong). Try again:");
                        }
                    },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find a closing bracket return the current node
                Token::Close => { current = 0; return Ok((node, index)) },
            }
            index += 1;
        }
        Ok((node, index))
    }

    /// Determines if this tree is just a leaf node.
    fn is_leaf(&self) -> bool {
        match *self {
            Expr::Leaf(_) => true,
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

    // Evaluate the tree recursively
    pub fn evaluate(&self) -> Option<i64> {
        match *self {
            Expr::Leaf(num) => Some(num.data()),
            Expr::Internal{ ref children, data } => {
                let num_one = match children[0].evaluate() {
                    Some(ok) => ok,
                    None => return None,
                };
                let num_two = match children[1].evaluate() {
                    Some(ok) => ok,
                    None => return None,
                };
                Some(data.op().apply(num_one, num_two))
            }
        }
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

    pub fn apply(&self, num_one: i64, num_two: i64) -> i64 {
        match self.data() {
            '+' => num_one + num_two,
            '-' => num_one - num_two,
            '*' => num_one * num_two,
            '/' => num_one / num_two,
            '%' => num_one % num_two,
            _ => 0,
        }
    }

    fn set_operation(&mut self, c: char) {
        self.op = c;
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Operation { op: Op },
    Open,
    Close,
    Number(i64),
}

impl Token {

    fn op(&self) -> Op {
        match *self {
            Token::Operation{ op } => op,
            _ => Op{ op: '+' },
        }
    }

    // Get numerical data if possible
    fn data(&self) -> i64 {
        match *self {
            Token::Number(data) => data,
            _ => 0,
        }
    }

    // Get character data if possible
    fn op_data(&self) -> char {
        match *self {
            Token::Operation{ op } => op.op,
            _ => ' ',
        }
    }

    fn set_operation(&mut self, c: char) {
        match *self {
            Token::Operation { op } => *self = Token::Operation { op: Op{ op: c }},
            _ => {},
        };
    }

    fn set_number(&mut self, n: i64) {
        match *self {
            Token::Number(before) => *self = Token::Number((before * 10) + n),
            _ => {},
        };
    }

    // was missing, compiler complained
    fn to_string(&self) -> String {
        match *self {
            Token::Number(data) => data.to_string(),
            Token::Open => "(".to_string(),
            Token::Close => ")".to_string(),
            Token::Operation{ op } => op.op.to_string(),
        }
    }
}
