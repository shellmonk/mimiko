#![allow(dead_code)]

use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StateMachine {
    states: HashMap<StateId, State>,
    initial_state: Option<StateId>,
    accept_state: Option<StateId>,
    pub current_state: Option<StateId>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial_state: None,
            accept_state: None,
            current_state: None,
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.states.insert(state.id, state);
    }

    // I wish this is C
    pub fn set_initial(&mut self, initial: StateId) {
        self.initial_state = Some(initial);
        self.current_state = self.initial_state;
    }

    pub fn set_accept(&mut self, accept: StateId) {
        self.accept_state = Some(accept);
    }

    pub fn process(&mut self, symbol: Symbol) -> bool {
        self.current_state = Option::from(
            self.states
                .get(&self.current_state.unwrap())
                .unwrap()
                .next(symbol),
        );

        if self.current_state.unwrap().0 == self.accept_state.unwrap().0 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Symbol(char);

#[derive(Debug)]
pub struct State {
    pub id: StateId,
    pub transitions: HashMap<Symbol, StateId>,
}

impl State {
    fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            id: StateId(rng.random::<u16>()),
            transitions: HashMap::new(),
        }
    }

    // this looks like Python
    pub fn add_transition(&mut self, symbol: Symbol, state: StateId) {
        self.transitions.insert(symbol, state);
    }

    // this is so unsafe
    pub fn next(&self, symbol: Symbol) -> StateId {
        self.transitions[&symbol]
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct StateId(u16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sm_1() {
        let mut sm = StateMachine::new();
        let mut s1 = State::new();
        let s1_id = s1.id;
        let mut s2 = State::new();
        let s2_id = s2.id;
        let s3 = State::new();
        let s3_id = s3.id;

        s1.add_transition(Symbol('a'), s2_id);
        s1.add_transition(Symbol('b'), s3_id);
        s2.add_transition(Symbol('c'), s3_id);
        sm.add_state(s1);
        sm.add_state(s2);
        sm.add_state(s3);

        sm.set_initial(s1_id);
        sm.set_accept(s3_id);

        let mut events = "ac".chars().into_iter();

        loop {
            let e = events.next();
            //println!("PROCESSING SYMBOL: {:?}", e);
            if e == Option::None {
                break;
            }
            if !sm.process(Symbol(e.unwrap())) {
                //println!("CURRENT STATE: {:?}", sm.current_state);
            } else {
                //println!("CURRENT STATE IS FINAL: {:?}", sm.current_state);
                break;
            }
        }
        //println!("STATE MACHINE: {:?}", sm);

        assert_eq!("1", "1");
    }
}
