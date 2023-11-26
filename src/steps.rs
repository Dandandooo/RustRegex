use automata::{NFA, DFA};
pub fn make_dfa(regex: String) -> DFA {

    step4(step3(step2(step1(regex))))
}

// @param regex looks like: "/[a-z]+/i", where the part between the slashes is the regex,
// and the last characters are the flags.
// Goal of step 1 is to convert the regex to a NFA with epsilon connections;
fn step1(regex: String) -> NFA {
    NFA::from(regex)
}

// @param nfa is the NFA from step1.
// Goal of step 2 is to convert the NFA to a NFA without epsilon connections;
fn step2(nfa: NFA) -> NFA {

    todo!();
}

// @param nfa is the NFA from step2.
// Goal of step 3 is to convert the NFA to a DFA by reworking duplicate connections;
fn step3(nfa: NFA) -> DFA {
    todo!();
}

// @param dfa is the DFA from step3.
// Goal of step 4 is to remove redundant & unreachable nodes.
fn step4(dfa: DFA) -> DFA {
    let mut dfa2 = dfa.clone();
    dfa2.purge();
    dfa2
}