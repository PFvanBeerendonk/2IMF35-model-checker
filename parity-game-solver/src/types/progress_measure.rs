use super::vertex::Vertex;

pub type Measures = Option<Vec<i64>>;
// In lectures refered to as M^T OR \N^d \union \{T\} (the set of tuples of natural numbers of length d and T)
// Here `Option` will be NULL or Vec<i64>, we will interpret NULL as T (see lecture 8, slide 6)

// Specify custom type `ProgressMeasure` to hide implementation details
pub struct ProgressMeasure {
    pub data: Vec<Measures>,
}


/* NOTE: The ProgressMeasure datatype should be able to do the following things efficiently:
 * - build, store and operate on the progress mater
 * 
 * More precisely
 * - find v \in V such that ϱ1 < Liftv (ϱ)
 * - update ϱ(v)
 * - find Prog(ϱ, v, w)
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

        let data = (0..id).map(|_| Some(vec![0; d as usize])).collect::<Vec<Measures>>();

        return Self{
            data: data
        }
    }

    /**
     * Should set ϱ(v) to new_value
     */
    pub fn set(self, vertex: i64, new_value: &str) -> Self {
        
        return self
    }


    /**
     * Prog (ϱ, v , w ), for v , w ∈ V , is the least m ∈ M⊤, such that:
     * • if p(v ) is even, then m ≥p(v ) ϱ(w )
     * • if p(v ) is odd, then either m >p(v ) ϱ(w ), or, if ϱ(w ) = ⊤, also m = ⊤
     * 
     * NOTE that (v,w) \in E
     */
    pub fn prog(self, v: Vertex, w: Vertex) -> Measures {
        if (v.priority.is_even()) {
            // get least m with m >=_{p(v)} ϱ(w)
            return None;

        } else {
            if (self.data[w.identifier].is_none()) {
                
            }

            // get least m >_{p(v)} ϱ(w) OR e(w) = T then m=T
            return None;
        }
    }
}
