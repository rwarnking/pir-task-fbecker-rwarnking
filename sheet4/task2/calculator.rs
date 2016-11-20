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
       test_tree(&input);
    }
}

// Makes a tree from the given string and evaluates it if possible
fn test_tree(line: &str) {
    match tokenize(line) {
        Ok(calc) => match Expr::parse(&calc, 0) {
            Ok((tree, _)) => if let Some(result)=tree.evaluate(0){println!("Result: {}", result)},
            Err(why) => println!("{}", why),
        },
        Err(why) => println!("{}", why),
    };
}

// Split input into Tokens
fn tokenize(line: &str) -> Result<Vec<Token>, &str> {
    let mut ret = Vec::new();
    let mut result = line.split_whitespace();
    // Go through every character in every string
    while let Some(elem) = result.next() {
        for c in elem.chars() {
            match c {
                '(' => ret.push(Token::Open( '(' )),
                ')' => ret.push(Token::Close( ')' )),
                '+' | '-' | '*' | '/' | '%' => ret.push(Token::Operation(c)),
                _ => if let Some(num) = c.to_digit(10) {
                    ret.push(Token::Number(num as i32))
                } else { return Err("Something went wrong when trying to parse a number!") },
            }
        }
    };

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
        let mut node = Expr::Leaf(Token::Operation('+'));
        // 0 = nothing, 1 = first number, 2 = second number, 3 = operator <= controls for checking if the input is valid
        let mut current = 0;

        while index < tokens.len() {
            match tokens[index] {
                // If we find a number, and the previous Token was an operator or
                // this is the first Token, add a leaf to the tree
                Token::Number(i32) => match current {
                    0 | 2 => { current = 1; node.add_leaf_child(tokens[index]) },
                    3 => { current = 2; node.add_leaf_child(tokens[index]) },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find an operator and the previous Token was a number change the node data
                Token::Operation(char) => match current {
                    1 => { current = 3; node.data_mut().set_operation(tokens[index]) },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find an opening bracket, and the previous Token was an operator or
                // this is the first Token, add a leaf to the tree through recursion
                Token::Open(char) => match current {
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
                Token::Close(char) => { current = 0; return Ok((node, index)) },
            }
            index += 1;
        }
        Ok((node, index))
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

    // Evaluate the tree recursively
    pub fn evaluate(&self, mut result: i32) -> Option<i32> {
        // Look at both children and return result depending on whether they are nested or not
        if let Some(next) = self.children() {
            // If there are two children
            if next.len() > 1 {
                match next[0].is_leaf() {
                    true => match next[1].is_leaf() {
                        true => if let Ok(num_one) = next[0].data().number() {
                            if let Ok(num_two) = next[1].data().number() {
                                if let Ok(op) = self.data().char_data() {
                                    return Some(result+Op::operation(op).apply(num_one, num_two));
                                }
                            }
                        },
                        false => if let Ok(num_one) = next[0].data().number() {
                            if let Ok(op) = self.data().char_data() {
                                if let Some(num_two) = next[1].evaluate(result) {
                                    return Some(result+Op::operation(op).apply(num_one, num_two));
                                }
                            }
                        },
                    },
                    false => match next[1].is_leaf() {
                        true => if let Ok(num_two) = next[1].data().number() {
                            if let Ok(op) = self.data().char_data() {
                                if let Some(num_one) = next[0].evaluate(result) {
                                    return Some(result+Op::operation(op).apply(num_one, num_two));
                                }
                            }
                         },
                        false => if let Ok(op) = self.data().char_data() {
                            if let Some(num_one) = next[0].evaluate(result) {
                                if let Some(num_two) = next[1].evaluate(result) {
                                    return Some(result+Op::operation(op).apply(num_one, num_two));
                                }
                            }
                        },
                    },
                }
            // If there is only one child
            } else if next[0].is_leaf() {
                if let Ok(num) = next[0].data().number() {
                    return Some(num);
                }
            } else if let Ok(op) = self.data().char_data() {
                if let Some(num) = next[0].evaluate(result) {
                    return Some(num);
                }
            }
        };
        None
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

    fn number(&self) -> Result<i32, &str> {
        match *self {
            Token::Number(data) => Ok(data),
            _ => Err("Tried to get a number from a Token not containing a number."),
        }
    }

    fn char_data(&self) -> Result<char, &str> {
        match *self {
            Token::Open(data) | Token::Close(data) | Token::Operation(data) => Ok(data),
            _ => Err("Tried to get a character from a Token not containing a character."),
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
