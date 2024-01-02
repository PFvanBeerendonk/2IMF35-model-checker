use std::collections::HashSet;

// Specify custom type `ProgressMeasure` to hide implementation details
pub struct ProgressMeasure {
    pub data: Vec<Vec<i64>>,
}

// TODO: inner datastruct Vec<i64> should becomde Vec<i64> OPTION T where T is a special constant datastruct



/* NOTE: The ProgressMeasure datatype should be able to do the following things efficiently:
 * - build, store and operate on the progress mater
 * 
 * More precisely
 * - find v \in V such that ϱ1 < Liftv (ϱ)
 * - update ϱ(v)
 *  */ 
impl ProgressMeasure{
    /**
     * Initialize ϱ(v) = d-tuples of element 0 FOREACH v \in V
     * 
     * id: maximal identifier (i.e. nr_of_states)
     * d: maximal priority + 1
     */
    pub fn new(id: i64, d: i64) -> Self{
        // note that all identifiers of vertices are between 0 and id (inclusive)

        let max_prio: usize = d as usize;

        let mut data = (0..id).map(|_| vec![0; max_prio]).collect::<Vec<Vec<i64>>>();

        return Self{
            data: data
        }
    }

    /**
     * Should set ϱ(v) to new_value
     */
    pub fn set(&mut self, vertex: i64, new_value: &str) -> &mut Self {
        
        return self
    }


    /**
     * Prog (ϱ, v , w ), for v , w ∈ V , is the least m ∈ M⊤, such that:
     * • if p(v ) is even, then m ≥p(v ) ϱ(w )
     * • if p(v ) is odd, then either m >p(v ) ϱ(w ), or, if ϱ(w ) = ⊤, also m = ⊤
     */
    pub fn prog(self, v: i64, w: i64) -> Self {
        return self
    }
}
