#[cfg(test)]
mod formula_tests {
    use std::fs;
    use model_checker::types::formula::{print_ast, parse_logic};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    fn test_print_ast(file_path: &str, expected_output: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents: String = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let mut input = contents
            .lines()
            .filter(|&line| !line.trim_start().starts_with('%'))
            .collect::<Vec<&str>>()
            .join("\n");
        input.retain(|c| (!c.is_whitespace() && c != '\n'));
        let parsed_formula = parse_logic(&input);
        let output = print_ast(&parsed_formula, 0);

        // Compare the actual output with the expected output
        println!("File{} converted from {} to:\n {}", file_path, input, output);
        assert_eq!(output, expected_output);

        Ok(())
    }

    fn test_files_in_directory(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
            if let Some(ext) = entry.path().extension() {
                if ext == "mcf" {
                    let file_path = entry.path();

                    // Construct the expected output file path
                    let mut expected_file_path = PathBuf::from(file_path);
                    expected_file_path.set_extension("expected");

                    // Check if the expected output file exists
                    if expected_file_path.exists() {
                        let expected_output = fs::read_to_string(&expected_file_path)?;

                        test_print_ast(file_path.to_str().unwrap(), &expected_output)?;
                    } else {
                        // Handle case where the expected file doesn't exist
                        println!("Expected file not found for {:?}", file_path);
                        let expression = fs::read_to_string(file_path.to_str().unwrap()).expect("Error reading contents of file");
                        print_ast(&parse_logic(&expression), 0);
                        // Perform necessary actions if the file doesn't exist (e.g., skip the test or fail the test)
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_files_in_tests_directory() {
        let tests_directory = "./../input/part1-tests/";

        if let Err(err) = test_files_in_directory(tests_directory) {
            // Fail the test in case of an error
            panic!("Test failed: {:?}", err);
        }
    }
}
