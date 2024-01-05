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
    use parity_game_solver::types::vertex::Vertex;
    
    // NOTE: use examples from lecture 8, slide 17
    // v.id = 0 ; w.id = 1 ; d=1+3
    // 
    #[test]
    fn test_function_prog_lec8_slide17_example1() {
        // Vertex: id, prio, owner, succ
        let v = Vertex::new(
            0, 0, 1, 
            Vec::<i64>::from([]),
        );
        let w = Vertex::new(
            1, 1, 0, 
            Vec::<i64>::from([]),
        );
        let d = 4;

        let mut pm = ProgressMeasure::new(
            2, // max id
            3, // max prio
        );
        pm.data[1] = Some(vec![0,2,0,0]);

        let result = pm.prog(v, w, d);
        assert_eq!(result, Some(vec![0,0,0,0]));
    }

    #[test]
    fn test_function_prog_lec8_slide17_example2() {
        // Vertex: id, prio, owner, succ
        let v = Vertex::new(
            0, 1, 1, 
            Vec::<i64>::from([]),
        );
        let w = Vertex::new(
            1, 1, 0, 
            Vec::<i64>::from([]),
        );
        let d = 3;

        let mut pm = ProgressMeasure::new(
            2, // max id
            3, // max prio
        );
        pm.data[1] = Some(vec![0,2,0,0]); // e(w)

        let result = pm.prog(v, w, d);
        assert_eq!(result, None); // T
    }

    #[test]
    fn test_function_prog_lec8_slide17_example3() {
        // Vertex: id, prio, owner, succ
        let v = Vertex::new(
            0, 3, 1, 
            Vec::<i64>::from([]),
        );
        let w = Vertex::new(
            1, 1, 0, 
            Vec::<i64>::from([]),
        );
        let d = 3;

        let mut pm = ProgressMeasure::new(
            2, // max id
            3, // max prio
        );
        pm.data[1] = Some(vec![0,2,0,0]);

        let result = pm.prog(v, w, d);
        assert_eq!(result, Some(vec![0,2,0,1]));
    }

    #[test]
    fn test_function_prog_odd_t() {
        // Vertex: id, prio, owner, succ
        let v = Vertex::new(
            0, 3, 1, 
            Vec::<i64>::from([]),
        );
        let w = Vertex::new(
            1, 1, 0, 
            Vec::<i64>::from([]),
        );
        let d = 3;

        let mut pm = ProgressMeasure::new(
            2, // max id
            3, // max prio
        );
        pm.data[1] = None;

        let result = pm.prog(v, w, d);
        assert_eq!(result, None);
    }
}
