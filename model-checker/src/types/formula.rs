#[derive(Debug)]
pub enum Operator {
    SimpleFalse, // f = false
    SimpleTrue,  // f = true
    StateLabel, // f = p NEEDED?
    Negate, // f = ¬g
    Conjunction, // f = g1 ∧ g2
    Disjunction, // f = g1 ∨ g2
    DiamondModality, // f = [a]g
    BoxModality, // f = <a>g
    LeastFixpoint, // mu X
    GreatestFixpoint, // nu X
}

#[derive(Debug)]
pub enum Node {
    Variable(String), // X / Y / etc.
    Action(String), // Something from the set Act (i, plato, etc.)
    UnaryExpr { op: Operator }, // SimpleFalse, SimpleTrue, Negate
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

fn parse_logic(expression: &str) -> Node {
    let expression = expression.trim();
    let mut operator = Operator::SimpleFalse;
    if !expression.contains("(") {
        let mut action = "";
        let mut variable = "";
        let mut operator = Operator::SimpleFalse;
        if expression.contains("&&") || expression.contains("||") {
            let (first_part, operator, second_part) = get_junctions(expression);
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(parse_logic(first_part)),
                rhs: Box::new(parse_logic(second_part))
            };
        } else if expression.contains("[") && expression.contains("]") {
            operator = Operator::BoxModality;
            (action, variable) = extract_bracketed_strings(expression, "box");
        } else if expression.contains("<") && expression.contains(">") {
            operator = Operator::DiamondModality;
            (action, variable) = extract_bracketed_strings(expression, "diamond");
        } else {
            println!("TODO {}", expression);
        }
        if variable == "true" {
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(Node::Action((*action).to_string())),
                rhs: Box::new(Node::UnaryExpr{ op: Operator::SimpleTrue }),
            };
        } else if variable == "false" {
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(Node::Action((*action).to_string())),
                rhs: Box::new(Node::UnaryExpr{ op: Operator::SimpleFalse }),
            };
        }
        return Node::BinaryExpr {
            op: operator,
            lhs: Box::new(Node::Action((*action).to_string())),
            rhs: Box::new(Node::Variable((*variable).to_string())),
        };
    } else if expression.starts_with("(")  {
        let (first_part, operator, second_part) = get_junctions(expression);
        return Node::BinaryExpr {
            op: operator,
            lhs: Box::new(parse_logic(first_part)),
            rhs: Box::new(parse_logic(second_part))
        };
    } else {
        let mut lhs = Node::Variable("TODO".to_string());
        let and_index = check_junction_before_paren(expression, "&&");
        let or_index = check_junction_before_paren(expression, "||");
        if and_index != usize::MAX {
            operator = Operator::Conjunction;
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(parse_logic(&remove_brackets(expression[..and_index].to_string()))),
                rhs: Box::new(parse_logic(&remove_brackets(expression[and_index+2..].to_string())))
            }
        } else if or_index != usize::MAX {
            operator = Operator::Disjunction;
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(parse_logic(&remove_brackets(expression[..or_index].to_string()))),
                rhs: Box::new(parse_logic(&remove_brackets(expression[or_index+2..].to_string())))
            }
        } 
        if let Some(extracted) = extract_text_before_brackets(expression) {
            // do mu, nu, <>, [], &&, and ||
            if extracted.ends_with("&&") {
                operator = Operator::Conjunction;
                lhs = Node::Variable(extracted[..extracted.len()-2].to_string());
            } else if extracted.ends_with("||") {
                operator = Operator::Disjunction;
                lhs = Node::Variable(extracted[..extracted.len()-2].to_string());
            } else if extracted.starts_with("[") {
                println!("EIGENLIJK NIET MOGELIJK {} en {}", expression, extracted);
                let (action, variable) = extract_bracketed_strings(expression, "box");
                return Node::BinaryExpr {
                    op: Operator::BoxModality,
                    lhs: Box::new(Node::Action((*action).to_string())),
                    rhs: Box::new(parse_logic(variable)),
                };
            } else if extracted.starts_with("<") {
                println!("EIGENLIJK NIET MOGELIJK {} en {}", expression, extracted);
                let (action, variable) = extract_bracketed_strings(expression, "diamond");
                return Node::BinaryExpr {
                    op: Operator::DiamondModality,
                    lhs: Box::new(Node::Action((*action).to_string())),
                    rhs: Box::new(parse_logic(variable)),
                };
                // lhs = Node::Variable(extracted[0..3].to_string());
            } else if extracted.starts_with("mu") {
                operator = Operator::LeastFixpoint;
                lhs = Node::Variable(extracted[2..3].to_string());
            } else if extracted.starts_with("nu") {
                operator = Operator::GreatestFixpoint;
                lhs = Node::Variable(extracted[2..3].to_string());
            } else if extracted.starts_with("[") {
                operator = Operator::DiamondModality;
            } else if extracted.starts_with("<") {
                operator = Operator::BoxModality;
                // lhs = Node::Variable(extracted[0..3].to_string());
            } else {
                println!("outer {}, operator {:?}, lhs {:?}", extracted, operator, lhs);
                // parse_logic(extracted);
            }
        };
        if let Some(extracted) = extract_text_between_brackets(expression) {
            return Node::BinaryExpr { op: operator, lhs: Box::new(lhs), rhs: Box::new(parse_logic(extracted)) };
        };
    }
    println!("PANIC {}", expression);
    return Node::Variable("ERROR".to_string());
}

fn extract_text_before_brackets(text: &str) -> Option<&str> {
    let before_brackets = text.splitn(2, '(').next()?;

    Some(before_brackets)
}

fn extract_text_between_brackets(text: &str) -> Option<&str> {
    let start_idx = text.find('(')?;
    let end_idx = text.rfind(')')?;

    Some(&text[start_idx+1..end_idx])
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
        if open > 2 {
            if Some(&s[open - 1..close])?.starts_with(".") {
                // we know that it starts as a fixpoint (muV./nuV.) where V is a var
                return Some(&s[open - 4..close + 1]);
            }
        }
        return Some(&s[open + 1..close]);
    }
    None
}

fn get_junctions<'a>(s: &'a str) -> (&'a str, Operator, &'a str) {
    let mut operator = Operator::SimpleFalse;
    if let Some(first) = get_strings_between_brackets(s) {
        if s[first.len()+2..].starts_with("&&") {
            operator = Operator::Conjunction;
        } else if s[first.len()+2..].starts_with("||") {
            operator = Operator::Disjunction;
        } else{
            println!("There was no operator found at the start of the string {}", &s[first.len()..]);
        }
        if let Some(second) = get_strings_between_brackets(&s[first.len()..]) {
            return (first, operator, second);
        }
    } else if s.contains("&&") {
        let expressions: Vec<&str> = s.split("&&").collect();
        return (expressions[0], Operator::Conjunction, expressions[1]);
    } else if s.contains("||") {
        let expressions: Vec<&str> = s.split("||").collect();
        return (expressions[0], Operator::Disjunction, expressions[1]);
    }
    panic!("No conjunction or disjunction found");
}

fn check_junction_before_paren(input_string: &str, str_to_check: &str) -> usize {
    if let Some(pos) = input_string.find('(') {
        let substring_before_paren = &input_string[..pos];
        if let Some(index) = substring_before_paren.find(str_to_check) {
            return index as usize;
        }
    }
    usize::MAX
}

fn remove_brackets(mut input_string: String) -> String {
    if let Some(first_char) = input_string.chars().next() {
        if let Some(last_char) = input_string.chars().last() {
            if first_char == '(' && last_char == ')' {
                input_string.pop(); // Remove last character
                input_string.remove(0); // Remove first character
            }
        }
    }
    input_string
}

fn extract_bracketed_strings<'a>(
    input: &'a str,
    bracket_type: &str,
) -> (&'a str, &'a str) {
    let open_bracket = match bracket_type {
        "diamond" => '<',
        "box" => '[',
        _ => return ("", ""), // If an invalid bracket type is provided, return empty strings
    };

    if let Some(open_idx) = input.find(open_bracket) {
        let close_bracket = match open_bracket {
            '[' => ']',
            '<' => '>',
            _ => return ("", ""), // Invalid bracket type provided
        };

        if let Some(close_idx) = input.find(close_bracket) {
            let bracket = &input[open_idx + 1..close_idx];
            let content = &input[close_idx + 1..];
            return (bracket, content);
        }
    }

    ("", "")
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