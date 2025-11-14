use crate::circuit::Circuit;
use crate::pattern::{add_node, Pattern, PniType};
use crate::pattern::PniType::NodeOutput;

impl Pattern {

    pub fn build_full_adder(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let (c0, a, b) = (PniType::Input(0), PniType::Input(1), PniType::Input(2));
        let (s, c1) = (1, 4);
        let n0 = add_node(
            &mut pattern,
            "xor",
            vec![a, b]
        );
        add_node(
            &mut pattern,
            "xor",
            vec![c0, NodeOutput(n0)]
        );
        let n1 = add_node(
            &mut pattern,
            "and",
            vec![c0, NodeOutput(n0)]
        );
        let n2 = add_node(
            &mut pattern,
            "and",
            vec![a, b]
        );
        add_node(
            &mut pattern,
            "or",
            vec![NodeOutput(n1), NodeOutput(n2)]
        );
        let mut pattern = Pattern::new(
            3,
            vec![s, c1],
            pattern
        );
        pattern.set_name(&["c0", "a", "b"], &["s", "c1"]);
        pattern.set_description("c0 + a + b = c1 s");
        circuit.get_patterns_mut().insert("full_adder".to_string(), pattern);

    }
    pub fn build_half_adder(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let input = vec![PniType::Input(0), PniType::Input(1)];
        let a_output = add_node(&mut pattern, "xor", input.clone());
        let b_output = add_node(&mut pattern, "and", input.clone());
        let mut pattern = Pattern::new(
            2,
            vec![a_output, b_output],
            pattern
        );
        pattern.set_name(
            &["a", "b"],
            &["s", "c"]
        );
        circuit.get_patterns_mut().insert("half_adder".to_string(), pattern);
    }
}

