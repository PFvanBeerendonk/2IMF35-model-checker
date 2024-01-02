use std::collections::HashSet;

// Specify custom type `ProgressMeasure` to hide implementation details

pub struct ProgressMeasure {
    pub data: HashSet<i64>,
}

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
     * d: maximal priority
     */
    pub fn new(d: i64) -> Self{
        


        return Self{
            data: HashSet::new()
        }
    }

    /**
     * Should set ϱ(v) to new_value
     */
    pub fn set(&mut self, vertex: i64, new_value: str) -> &mut Self {
        
        return self
    }


    /**
     * Prog (ϱ, v , w ), for v , w ∈ V , is the least m ∈ M⊤, such that:
     * • if p(v ) is even, then m ≥p(v ) ϱ(w )
     * • if p(v ) is odd, then either m >p(v ) ϱ(w ), or, if ϱ(w ) = ⊤, also m = ⊤
     */
    pub fn prog(self, v: i64, w: i64) -> self {

    }
}
