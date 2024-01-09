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
        const D: i64 = 4;

        let mut pm = ProgressMeasure::new(
            2, // max id
            3, // max prio
        );
        pm.data[1] = Some(vec![0,2,0,0]);

        let result = pm.prog(v, w, D);
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
    use parity_game_solver::types::vertex::Vertices;
    
    /**
     * Construct example as given in lecture 8, slide 21/43
     */
    fn construct_example_lec8_slide21() -> (ProgressMeasure, Vertices) {
        let mut pm = ProgressMeasure::new(
            7, // nr of states
            4, // d
        );
        for i in 0..7 {
            pm.data[i] = Some(vec![0,0,0,0]);
        }

        const NONE: Option<Vertex> = None;
        let mut vertices: Vertices = Vec::from([NONE; 7]);

        // Vertex: id, prio, owner, succ
        // X
        vertices[0] = Some(Vertex::new(
            0, 1, 1, 
            vec![0, 1],
        ));
        // X'
        vertices[1] = Some(Vertex::new(
            1, 1, 0, 
            vec![2, 4],
        ));
        // Y
        vertices[2] = Some(Vertex::new(
            2, 2, 1, 
            vec![3, 6],
        ));
        // Y'
        vertices[3] = Some(Vertex::new(
            3, 2, 0, 
            vec![2, 0],
        ));
        // Z
        vertices[4] = Some(Vertex::new(
            4, 3, 0, 
            vec![5],
        ));
        // Z'
        vertices[5] = Some(Vertex::new(
            5, 3, 0, 
            vec![5],
        ));
        // W
        vertices[6] = Some(Vertex::new(
            6, 3, 0, 
            vec![6, 4],
        ));
        
        return (pm, vertices)
    }

    #[test]
    fn test_function_liftv_slide_example() {
        let construct = construct_example_lec8_slide21();
        let mut pm: ProgressMeasure = construct.0;
        let vertices: Vertices = construct.1;
        const D: i64 = 4;

        // X
        // vertex id, all vertices, d
        let mut  res = pm.lift_v(0, &vertices, D);
        pm = res.0;
        let mut changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        // rest is unchanged
        for i in 1..7 {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // Y', Y
        for v in [2,3] {
            res = pm.lift_v(v, &vertices, D);
            pm = res.0;
            changed = res.1;
            assert_eq!(changed, false);
            assert_eq!(pm.data[0], None);
            // rest is unchanged
            for i in 1..7 {
                assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
            }
        }

        // X'
        res = pm.lift_v(1, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], Some(vec![0,1,0,0]));
        // rest is unchanged
        for i in 2..7 {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }
        
        // Z'
        res = pm.lift_v(5, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], Some(vec![0,1,0,0]));
        assert_eq!(pm.data[5], None);
        // rest is unchanged
        for i in [2,3,4,6] {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // Z
        res = pm.lift_v(4, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], Some(vec![0,1,0,0]));
        assert_eq!(pm.data[4], None);
        assert_eq!(pm.data[5], None);
        // rest is unchanged
        for i in [2,3,6] {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // W
        res = pm.lift_v(6, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], Some(vec![0,1,0,0]));
        assert_eq!(pm.data[4], None);
        assert_eq!(pm.data[5], None);
        assert_eq!(pm.data[6], None);
        // rest is unchanged
        for i in [2,3] {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // Y
        res = pm.lift_v(2, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], Some(vec![0,1,0,0]));
        assert_eq!(pm.data[2], None);
        assert_eq!(pm.data[4], None);
        assert_eq!(pm.data[5], None);
        assert_eq!(pm.data[6], None);
        // rest is unchanged
        for i in [3] {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // X'
        res = pm.lift_v(1, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], None);
        assert_eq!(pm.data[2], None);
        assert_eq!(pm.data[4], None);
        assert_eq!(pm.data[5], None);
        assert_eq!(pm.data[6], None);
        // rest is unchanged
        for i in [3] {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // Y'
        res = pm.lift_v(3, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        assert_eq!(pm.data[1], None);
        assert_eq!(pm.data[2], None);
        assert_eq!(pm.data[3], None);
        assert_eq!(pm.data[4], None);
        assert_eq!(pm.data[5], None);
        assert_eq!(pm.data[6], None);
    }

    #[test]
    fn test_function_liftv_multiple_applications() {
        let construct = construct_example_lec8_slide21();
        let mut pm: ProgressMeasure = construct.0;
        let vertices: Vertices = construct.1;
        const D: i64 = 4;

        // X
        // vertex id, all vertices, d
        let mut res = pm.lift_v(0, &vertices, D);
        pm = res.0;
        let mut changed = res.1;
        assert_eq!(changed, true);
        assert_eq!(pm.data[0], None);
        // rest is unchanged
        for i in 1..7 {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }

        // X again should do nothing
        
        res = pm.lift_v(0, &vertices, D);
        pm = res.0;
        changed = res.1;
        assert_eq!(changed, false); // HAS NOT CHANGED!
        assert_eq!(pm.data[0], None);
        // rest is unchanged
        for i in 1..7 {
            assert_eq!(pm.data[i], Some(vec![0,0,0,0]));
        }
    }
}
