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


#[cfg(test)]
mod test_helper {
    use parity_game_solver::types::progress_measure::_tail_zeros;
    use parity_game_solver::types::progress_measure::_is_even;
    
    #[test]
    fn test_helper_tail_zeroes() {
        let v = vec![1,2,3,4,5];

        let result = _tail_zeros(v, 2);
        assert_eq!(result, vec![1,2,0,0,0]);
    }

    #[test]
    fn test_helper_tail_zeroes_2() {
        let v = vec![1,2,3,4];

        let result = _tail_zeros(v, 0);
        assert_eq!(result, vec![0,0,0,0]);
    }
    
    #[test]
    fn test_helper_is_even() {
        assert_eq!(true, _is_even(0));
        assert_eq!(false, _is_even(1));

        assert_eq!(true, _is_even(122392));
        assert_eq!(false, _is_even(33201));
    }
}


#[cfg(test)]
mod test_min_max_measures {
    use parity_game_solver::types::progress_measure::{min_measures, max_measures};
    
    // min measures
    #[test]
    fn test_function_min_measures_none() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            None,
            Some( vec![0,0,0,0] ),
        );
        let result = min_measures(list);
        assert_eq!(result, Some(vec![0,0,0,0]));
    }
    
    #[test]
    fn test_function_min_measures_none_2() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            None,
            Some( vec![0,2,0,1]),
        );
        let result = min_measures(list);
        assert_eq!(result, Some(vec![0,1,0,1]));
    }

    #[test]
    fn test_function_min_measures_all_none() {
        let list = vec!(
            None,
            None,
            None,
        );
        let result = min_measures(list);
        assert_eq!(result, None);
    }

    #[test]
    fn test_function_min_measures_same() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            Some( vec![0,1,0,1] ),
            Some( vec![0,1,0,1] ),
            None,
            Some( vec![0,2,0,1]),
            Some( vec![0,2,0,1]),
        );
        let result = min_measures(list);
        assert_eq!(result, Some(vec![0,1,0,1]));
    }
    
    // max measures
    #[test]
    fn test_function_max_measures() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            Some( vec![0,0,0,0] ),
        );
        let result = max_measures(list);
        assert_eq!(result, Some(vec![0,1,0,1]));
    }
    
    #[test]
    fn test_function_max_measures_none() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            None,
            Some( vec![0,2,0,1]),
        );
        let result = max_measures(list);
        assert_eq!(result, None);
    }

    #[test]
    fn test_function_max_measures_all_none() {
        let list = vec!(
            None,
            None,
            None,
        );
        let result = max_measures(list);
        assert_eq!(result, None);
    }

    #[test]
    fn test_function_max_measures_same() {
        let list = vec!(
            Some( vec![0,1,0,1] ),
            Some( vec![0,1,0,1] ),
            Some( vec![0,1,0,1] ),
            Some( vec![0,2,0,1]),
            Some( vec![0,2,0,1]),
            Some( vec![0,2,0,0]),
        );
        let result = max_measures(list);
        assert_eq!(result, Some(vec![0,2,0,1]));
    }
    
}


#[cfg(test)]
mod test_lift_v {
    use parity_game_solver::types::progress_measure::ProgressMeasure;
    use parity_game_solver::types::vertex::Vertex;
    
    #[test]
    fn test_function_liftv_lec8_slide17_example1() {
        // // Vertex: id, prio, owner, succ
        // let v = Vertex::new(
        //     0, 0, 1, 
        //     Vec::<i64>::from([]),
        // );
        // let w = Vertex::new(
        //     1, 1, 0, 
        //     Vec::<i64>::from([]),
        // );
        // let d = 4;

        // let mut pm = ProgressMeasure::new(
        //     2, // max id
        //     3, // max prio
        // );
        // pm.data[1] = Some(vec![0,2,0,0]);

        // let result = pm.lift_v(v);
        // assert_eq!(result, Some(vec![0,0,0,0]));
    }

}
