#[cfg(test)]
mod test_new {
    use parity_game_solver::types::progress_measure::ProgressMeasure;

    #[test]
    fn test_get_all_states() {
        let pm =  ProgressMeasure::new(
            2,
            4,
        );

        let expected_result = vec![
            Some(vec![0,0,0,0]),
            Some(vec![0,0,0,0]),
        ];

        assert_eq!(pm.data, expected_result);
    }
}


#[cfg(test)]
mod test_prog {
    use parity_game_solver::types::progress_measure::ProgressMeasure;
    

    #[test]
    fn test_function_prog() {
        let pm =  ProgressMeasure::new(
            2,
            4,
        );

        let expected_result = Some(vec![0,0,0,0]);

        let result = pm.prog(1,2);

        assert_eq!(result, expected_result);
    }
}
