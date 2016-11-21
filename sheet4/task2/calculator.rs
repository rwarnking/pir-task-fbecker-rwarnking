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
                Ok((tree, _)) => if let Some(result) = tree.evaluate(0) {
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
    let mut result = line.split_whitespace();
    // Go through every character in every string
    while let Some(elem) = result.next() {
        if elem == "exit"
        {
            return Err(1);
        }

        for c in elem.chars() {
            match c {
                '(' => ret.push(Token::Open( '(' )),
                ')' => ret.push(Token::Close( ')' )),
                '+' | '-' | '*' | '/' | '%' => ret.push(Token::Operation(c)),
                _ => if let Some(num) = c.to_digit(10) {
                    ret.push(Token::Number(num as i64))
                } else { return Err(0) },
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
                    0 => { current = 1; node.add_leaf_child(tokens[index]) },
                    i @ 1 | i @ 2  => if let Some(children) = node.children_mut() {
                        if !children[i-1].data_mut().set_number(tokens[index]) {
                            return Err("Could not set number with several digits. Try again:");
                        }
                    } else {
                            return Err("Could not change node children. Try again:");
                    },
                    3 => { current = 2; node.add_leaf_child(tokens[index]) },
                    _ => return Err("Input incorrect (maybe the order was wrong). Try again:"),
                },
                // If we find an operator and the previous Token was a number change the node data
                Token::Operation(char) => match current {
                    1 => if !node.data_mut().set_operation(tokens[index]) {
                        return Err("Could not set operator. Try again:")
                    } else { current = 3; },
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
    pub fn evaluate(&self, mut result: i64) -> Option<i64> {
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
                    return Some(num as i64);
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

    pub fn apply(&self, num_one: i64, num_two: i64) -> i64 {
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
    Number(i64),
}

impl Token {

    // Returns true if Token is an operation
    fn is_operation(&self) -> bool {
        match *self {
            Token::Operation(_) => true,
            _ => false,
        }
    }

    // Returns true if Token is a number
    fn is_number(&self) -> bool {
        match *self {
            Token::Number(_) => true,
            _ => false,
        }
    }

    // Get numerical data if possible
    fn number(&self) -> Result<i64, &str> {
        match *self {
            Token::Number(data) => Ok(data),
            _ => Err("Tried to get a number from a Token not containing a number."),
        }
    }

    // Get character data if possible
    fn char_data(&self) -> Result<char, &str> {
        match *self {
            Token::Open(data) | Token::Close(data) | Token::Operation(data) => Ok(data),
            _ => Err("Tried to get a character from a Token not containing a character."),
        }
    }

    // Reset operator, because we always start with a '+'
    fn set_operation(&mut self, t: Token) -> bool {
        match *self {
            Token::Number(_) => false,
            _ => { *self = t; true },
        }
    }

    // Change the number of a Token. Used when we get a number
    // with more than one digit.
    fn set_number(&mut self, t: Token) -> bool {
        match *self {
            Token::Number(before) => if let Ok(num) = t.number() {
                *self = Token::Number((before * 10) + num);
                true
            }
            else { println!("self.number() ging nicht"); false },
            _ => { println!("keine zahl");false },
        }
    }

    // was missing, compiler complained
    fn to_string(&self) -> String {
        match *self {
            Token::Number(data) => data.to_string(),
            Token::Open(data) | Token::Close(data) | Token::Operation(data) => data.to_string(),
        }
    }
}
