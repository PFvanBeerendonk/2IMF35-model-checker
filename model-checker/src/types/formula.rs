use std::collections::HashSet;

#[derive(Debug)]
pub enum Operator {
    SimpleFalse, // f = false
    SimpleTrue,  // f = true
    StateLabel, // f = p NEEDED?
    Negate, // f = ¬g NOT USED
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
    pub root_node: Node,
    pub variables: HashSet<String>, // all vars hashset
    pub actions: HashSet<String>, // all actions in the AST hashset
}

pub fn parse_logic(expression: &str) -> Node {
    let expression = expression.trim();
    let mut operator = Operator::SimpleFalse;
    if !expression.contains("(") {
        let action: &str;
        let variable: &str;
        if expression.contains("&&") || expression.contains("||") {
            let (first_part, operator, second_part) = get_junctions(expression);
            return Node::BinaryExpr {
                op: operator,
                lhs: Box::new(parse_logic(first_part)),
                rhs: Box::new(parse_logic(second_part))
            };
        } else if expression.starts_with("mu") {
            let rhs_idx = expression.find('.')
                .expect(". must be found");

            return Node::BinaryExpr {
                op: Operator::LeastFixpoint,
                lhs: Box::new(Node::Variable(expression[2..rhs_idx].to_string())),
                rhs: Box::new(parse_logic(&expression[rhs_idx + 1..])),
            };
        } else if expression.starts_with("nu") {
            let rhs_idx = expression.find('.')
                .expect(". must be found");
            return Node::BinaryExpr {
                op: Operator::GreatestFixpoint,
                lhs: Box::new(Node::Variable(expression[2..rhs_idx].to_string())),
                rhs: Box::new(parse_logic(&expression[rhs_idx+1..])),
            };
        } else if expression.starts_with("[") && expression.contains("]") {
            (action, variable) = extract_bracketed_strings(expression, "box");
            return Node::BinaryExpr {
                op: Operator::BoxModality,
                lhs: Box::new(Node::Action((*action).to_string())),
                rhs: Box::new(parse_logic(variable)),
            };
        } else if expression.starts_with("<") && expression.contains(">") {
            (action, variable) = extract_bracketed_strings(expression, "diamond");
            return Node::BinaryExpr {
                op: Operator::DiamondModality,
                lhs: Box::new(Node::Action((*action).to_string())),
                rhs: Box::new(parse_logic(variable)),
            };
        } else if expression == "true" {
            return Node::UnaryExpr { op: Operator::SimpleTrue }
        } else if expression == "false" {
            return Node::UnaryExpr { op: Operator::SimpleFalse }
        }
        return Node::Variable(expression.to_string());
    } else if expression.starts_with("(")  {
        let and_index = check_junction_after_paren(expression, "&&");
        let or_index = check_junction_after_paren(expression, "||");
        if and_index != usize::MAX {
            return Node::BinaryExpr {
                op: Operator::Conjunction,
                lhs: Box::new(parse_logic(&remove_brackets(expression[..and_index].to_string()))),
                rhs: Box::new(parse_logic(&remove_brackets(expression[and_index+2..].to_string())))
            }
        } else if or_index != usize::MAX {
            return Node::BinaryExpr {
                op: Operator::Disjunction,
                lhs: Box::new(parse_logic(&remove_brackets(expression[..or_index].to_string()))),
                rhs: Box::new(parse_logic(&remove_brackets(expression[or_index+2..].to_string())))
            }
        }
        let (first_part, operator, second_part) = get_junctions(expression);
        return Node::BinaryExpr {
            op: operator,
            lhs: Box::new(parse_logic(first_part)),
            rhs: Box::new(parse_logic(second_part))
        };
    } else {
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
            // do mu, nu, <>, and []
            if extracted.starts_with("[") {
                let (action, variable) = extract_bracketed_strings(expression, "box");
                return Node::BinaryExpr {
                    op: Operator::BoxModality,
                    lhs: Box::new(Node::Action((*action).to_string())),
                    rhs: Box::new(parse_logic(variable)),
                };
            } else if extracted.starts_with("<") {
                let (action, variable) = extract_bracketed_strings(expression, "diamond");
                return Node::BinaryExpr {
                    op: Operator::DiamondModality,
                    lhs: Box::new(Node::Action((*action).to_string())),
                    rhs: Box::new(parse_logic(variable)),
                };
                // lhs = Node::Variable(extracted[0..3].to_string());
            } else if extracted.starts_with("mu") {
                let rhs_idx = expression.find('.')
                    .expect(". must be found");
    
                return Node::BinaryExpr {
                    op: Operator::LeastFixpoint,
                    lhs: Box::new(parse_logic(&extracted[2..rhs_idx])),
                    rhs: Box::new(parse_logic(&remove_brackets(expression[rhs_idx + 1..].to_string()))),
                };
            } else if extracted.starts_with("nu") {
                let rhs_idx = expression.find('.')
                    .expect(". must be found");
    
                return Node::BinaryExpr {
                    op: Operator::GreatestFixpoint,
                    lhs: Box::new(parse_logic(&extracted[2..rhs_idx])),
                    rhs: Box::new(parse_logic(&remove_brackets(expression[rhs_idx + 1..].to_string()))),
                };
            } else {
                // parse_logic(extracted);
            }
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
    let operator: Operator;
    if let Some(first) = get_strings_between_brackets(s) {
        // if the first part between bracket has the length of the full string
        if s.len() == first.len() + 2 {
            // get the junction within that pair of brackets
            if s.contains("&&") {
                let expressions: Vec<&str> = s.split("&&").collect();
                return (&expressions[0][1..], Operator::Conjunction, &expressions[1][..expressions[1].len()-1]);
            } else if s.contains("||") {
                let expressions: Vec<&str> = s.split("||").collect();
                return (&expressions[0][1..], Operator::Disjunction, &expressions[1][..expressions[1].len()-1]);
            } else {
                panic!("There was no operator in the string {}", &s);
            }
        } else if s[first.len()+2..].starts_with("&&") {
            operator = Operator::Conjunction;
        } else if s[first.len()+2..].starts_with("||") {
            operator = Operator::Disjunction;
        } else {
            panic!("There was no operator found at the start of the string {}, {}, {}", &s[first.len()..], s.len(), first.len());
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
    panic!("No conjunction or disjunction found {}", s);
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

fn check_junction_after_paren(input_string: &str, str_to_check: &str) -> usize {
    if let Some(pos) = input_string.rfind(')') {
        let substring_after_paren = &input_string[pos..];
        if input_string.len() > pos+1 {
            if let Some(index) = substring_after_paren.find(str_to_check) {
                return index+pos as usize;
            }
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

fn extract_variables_from_node(node: &Node, variables: &mut HashSet<String>) {
    match node {
        Node::Variable(var) => {
            variables.insert(var.clone());
        }
        Node::BinaryExpr { lhs, rhs, .. } => {
            extract_variables_from_node(lhs, variables);
            extract_variables_from_node(rhs, variables);
        }
        Node::UnaryExpr { .. } | Node::Action(_) => {}
    }
}

fn extract_variables(root_node: &Node) -> HashSet<String> {
    let mut variables = HashSet::new();
    extract_variables_from_node(root_node, &mut variables);
    variables
}

fn extract_actions_from_node(node: &Node, actions: &mut HashSet<String>) {
    match node {
        Node::Action(action) => {
            actions.insert(action.clone());
        }
        Node::BinaryExpr { lhs, rhs, .. } => {
            extract_actions_from_node(lhs, actions);
            extract_actions_from_node(rhs, actions);
        }
        Node::UnaryExpr { .. } | Node::Variable(_) => {}
    }
}

fn extract_actions(root_node: &Node) -> HashSet<String> {
    let mut actions = HashSet::new();
    extract_actions_from_node(root_node, &mut actions);
    actions
}



pub fn print_ast(node: &Node, indent: usize) -> String {
    let mut output = String::new();

    match node {
        Node::Variable(var) => {
            output.push_str(&format!("{:indent$}Variable({})\n", "", var, indent = indent));
        }
        Node::Action(act) => {
            output.push_str(&format!("{:indent$}Action({:?})\n", "", act, indent = indent));
        }
        Node::UnaryExpr { op } => {
            output.push_str(&format!("{:indent$}UnaryExpr({:?})\n", "", op, indent = indent));
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            output.push_str(&format!("{:indent$}BinaryExpr({:?})\n", "", op, indent = indent));
            output.push_str(&print_ast(lhs, indent + 4));
            output.push_str(&print_ast(rhs, indent + 4));
        }
    }
    output
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
        println!("{}", print_ast(&parsed_formula, 0));
        
        return Self{
            variables: extract_variables(&parsed_formula),
            actions: extract_actions(&parsed_formula),
            root_node: parsed_formula,
        }
    }
}