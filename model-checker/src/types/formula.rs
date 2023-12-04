#[derive(Debug)]
pub enum Operator {
    SimpleFalse, // f = false
    SimpleTrue,  // f = true
    Predicate(String), // f = p
    Negate, // f = ¬g
    Conjunction, // f = g1 ∧ g2
    Disjunction, // f = g1 ∨ g2
    DiamondModality(String, Box<Node>), // f = [a]g
    BoxModality(String, Box<Node>), // f = <a>g
    LeastFixpoint, // mu
    GreatestFixpoint, // nu
}

#[derive(Debug)]
pub enum Node {
    Variable(String),
    Operator(Operator),
    UnaryExpr { op: Operator, child: Box<Node> },
    BinaryExpr { op: Operator, lhs: Box<Node>, rhs: Box<Node> },
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

fn parse_logic(expression: &str) {
    let expression = expression.trim();

    if !expression.contains("(") {
        println!("TODO {}", expression);
    } else if expression.starts_with("(")  {
        println!("the string {}", expression);
        let (first_part, second_part) = get_junctions(expression);
        println!("Part1: {:}, Part2: {:}", first_part, second_part);
        parse_logic(first_part);
        parse_logic(second_part);
    } else {
        if let Some(extracted) = extract_text_between_brackets(expression) {
            println!("inner {}", extracted); // Output: this(getThis)else
            parse_logic(extracted);
        };
        if let Some(extracted) = extract_text_before_brackets(expression) {
            println!("outer {}", extracted); // Output: this(getThis)else
            parse_logic(extracted);
        };

    }
}

fn extract_text_before_brackets(text: &str) -> Option<&str> {
    let before_brackets = text.splitn(2, '(').next()?;

    Some(before_brackets)
}

fn extract_text_between_brackets(text: &str) -> Option<&str> {
    let start_idx = text.find('(')?;
    let mut open_count = 0;

    let end_idx = text[start_idx..]
        .char_indices()
        .find_map(|(i, c)| {
            if c == '(' {
                open_count += 1;
            } else if c == ')' {
                open_count -= 1;
                if open_count == 0 {
                    return Some(start_idx + i);
                }
            }
            None
        })?;

    Some(&text[start_idx+1..=end_idx-1])
}

fn get_strings_between_brackets(s: &str) -> Option<&str> {
    let mut stack = Vec::new();
    let mut open_index = None;
    let mut close_index = None;

    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                if stack.is_empty() {
                    open_index = Some(i);
                }
                stack.push(c);
            }
            ')' => {
                if let Some('(') = stack.pop() {
                    if stack.is_empty() {
                        close_index = Some(i);
                        break;
                    }
                }
            }
            _ => continue,
        }
    }

    if let (Some(open), Some(close)) = (open_index, close_index) {
        if open > 0 {
            if Some(&s[open - 1..close])?.starts_with(".") {
                // we know that it starts as a fixpoint (muV./nuV.) where V is a var
                return Some(&s[open - 4..close]);
            }
        }
        return Some(&s[open + 1..close]);
    }
    None
}

fn get_junctions(s: &str) -> (&str, &str) {
    if let Some(first) = get_strings_between_brackets(s) {
        println!("First string between brackets: {}", first);
        if let Some(second) = get_strings_between_brackets(&s[first.len()..]) {
            println!("Second string between brackets in the previous section: {}", second);
            return (first, second);
        }
    }
    panic!("No conjunction or disjunction found");
}

impl Formula {
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

        let parsed_formula = parse_logic(&formula);
        println!("{:?}", parsed_formula);
        
        return Self{
            RootNode: ("TODO").to_string(),
            variables: vec!["-w".to_string(), "60".to_string(), "arg".to_string()],
        }
    }
}