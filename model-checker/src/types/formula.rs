#[derive(Debug)]
pub enum Operator {
    SimpleFalse, // f = false
    SimpleTrue, // f = true
    EqualP, // f = p
    Negate, // f = ¬g
    Conjunction, // f = g1 ∧ g2
    Disjunction, // f = g1 ∨ g2
    DiamondModality, // f = [a]g
    BoxModality, // f = <a>g
    LeastFixpoint, // mu
    GreatestFixpoint, // nu
}

pub enum Node {
    Variable(String),
    Operator(Operator),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

pub struct Formula {
    pub variables: Vec<String>, // all vars array

    pub RootNode: String,
    // f f = Xi then return A[i]
    //         else if f = true then return S
    //         else if f = p then return {s ∈ S | p ∈ L(s)}
    //         else if f = ¬g then return S \ eval(g )
    //         else if f = g1 ∧ g2 then return eval(g1) ∩ eval(g2)
    //         else if f = [a]g then return {s ∈ S | ∀t ∈ S : s a −→ t ⇒ t ∈ eval(g )}
    //         else if f = νXi .g (Xi ) then
}

fn parse_formula(formula: &str) -> Node {
    let mut iter = formula.chars().peekable();
    parse_expression(&mut iter)
}

fn parse_expression(iter: &mut std::iter::Peekable<std::str::Chars>) -> Node {
    let mut node = parse_term(iter);

    loop {
        match iter.peek() {
            Some(&c) if c == '&' || c == '|' => {
                iter.next(); // Consume the operator ('&' or '|')
                let op = match c {
                    '&' => Operator::Conjunction,
                    '|' => Operator::Disjunction,
                    _ => unreachable!(),
                };
                let rhs = parse_term(iter);
                node = Node::BinaryExpr {
                    op,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    node
}

fn parse_term(iter: &mut std::iter::Peekable<std::str::Chars>) -> Node {
    match iter.peek().cloned() {
        Some('[') => {
            iter.next(); // Consume '['
            let op = match iter.next() {
                Some('i') => Operator::DiamondModality,
                _ => panic!("Invalid operator"),
            };
            let var = parse_variable(iter);
            Node::UnaryExpr {
                op,
                child: Box::new(Node::Variable(var)),
            }
        }
        Some('m') => {
            iter.next(); // Consume 'm'
            let op = Operator::LeastFixpoint;
            let var = parse_variable(iter);
            Node::UnaryExpr {
                op,
                child: Box::new(Node::Variable(var)),
            }
        }
        Some('n') => {
            iter.next(); // Consume 'n'
            let op = Operator::GreatestFixpoint;
            let var = parse_variable(iter);
            Node::UnaryExpr {
                op,
                child: Box::new(Node::Variable(var)),
            }
        }
        Some('t') => {
            iter.next(); // Consume 't'
            Node::Operator(Operator::SimpleTrue)
        }
        Some('f') => {
            iter.next(); // Consume 'f'
            Node::Operator(Operator::SimpleFalse)
        }
        Some(_) => {
            let var = parse_variable(iter);
            Node::Variable(var)
        }
        None => panic!("Unexpected end of formula"),
    }
}

fn parse_variable(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut var = String::new();
    while let Some(&c) = iter.peek() {
        if c.is_alphanumeric() {
            var.push(c);
            iter.next();
        } else {
            break;
        }
    }
    var
}

fn print_ast(node: &Node, indent: usize) {
    match node {
        Node::Variable(var) => println!("{:indent$}Variable({})", "", var, indent = indent),
        Node::Operator(op) => println!("{:indent$}Operator({:?})", "", op, indent = indent),
        Node::UnaryExpr { op, child } => {
            println!("{:indent$}UnaryExpr({:?})", "", op, indent = indent);
            print_ast(child, indent + 4);
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            println!("{:indent$}BinaryExpr({:?})", "", op, indent = indent);
            print_ast(lhs, indent + 4);
            print_ast(rhs, indent + 4);
        }
    }
}


impl Formula{
    pub fn new(input_formula: String) -> Self {
        println!("Creating new formula");
        
        // format the string
        let mut formula = input_formula
            .lines()
            .filter(|&line| !line.trim_start().starts_with('%'))
            .collect::<Vec<&str>>()
            .join("\n");
        formula.retain(|c| (!c.is_whitespace() && c != '\n'));


        println!("{}", formula);

        let parsed_formula = parse_formula(&formula);
        print_ast(&parsed_formula, 0);
        
        return Self{
            RootNode: ("TODO").to_string(),
            variables: vec!["-w".to_string(), "60".to_string(), "arg".to_string()],
        }
    }
}