use array_tool::vec::Union as ATUnion;
use array_tool::vec::Intersect as ATIntersect;


/**
 * Defines a set of states, if `all_states` is true then `states` is None
 * This will save us returning a HUGE vector
 * 
 * #returns 
 * - `bool`: all_states
 * - `states`: `bool` is false -> Vec<i64>
 */
pub struct StateSet {
    pub all_states: bool, 
    pub states: Option<Vec<i64>>
}

pub trait Union {
    fn union(&self, other: Self) -> Self;
}
impl Union for StateSet { 
    fn union(&self, r:StateSet) -> StateSet {
        if self.all_states || r.all_states {
            return StateSet{all_states:true,  states:None} 
        }
        if self.states == None || r.states == None {
            panic!("If all_states is false, states cannot be None")
        }

        let return_value = self.states.clone().unwrap().union(r.states.unwrap());

        return StateSet{all_states:false,  states:Some(return_value)} 
    }
}

pub trait Intersect {
    fn intersect(&self, other: Self) -> Self;
}
impl Intersect for StateSet {
    fn intersect(&self, r:StateSet) -> StateSet {
        if self.all_states {
            return r;
        }
        if r.all_states {
            return StateSet{all_states:self.all_states, states:self.states.clone()};
        }
        if self.states == None || r.states == None {
            panic!("If all_states is false, states cannot be None")
        }

        let return_value = self.states.clone().unwrap().intersect(r.states.unwrap());

        return StateSet{all_states:false,  states:Some(return_value)}
    }
}
