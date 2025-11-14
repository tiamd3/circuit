use crate::circuit::Circuit;
use crate::pattern::{add_node, Pattern, PniType};
use crate::pattern::PniType::NodeOutput;

impl Pattern {
    pub fn build_jkff(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let (k, cp, j) = (PniType::Input(0), PniType::Input(1), PniType::Input(2));
        let (nq, q) = (2, 3);
        let n0 = add_node(
            &mut pattern,
            "nand",
            vec![k, cp, NodeOutput(q)]
        );
        let n1 = add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(nq), cp, j]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n0), NodeOutput(q)]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n1), NodeOutput(nq)]
        );
        let mut pattern = Pattern::new(
            3,
            vec![nq, q],
            pattern
        );
        pattern.set_name(&["k", "cp", "j"], &["nq", "q"]);
        pattern.set_description("Qn+1 = (J and not(Qn)) or (not(K) and Qn)");
        circuit.get_patterns_mut().insert("jkff".to_string(), pattern);

    }
    pub fn build_tff(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let (cp, t) = (PniType::Input(0), PniType::Input(1));
        let (nq, q) = (2, 3);
        let n0 = add_node(
            &mut pattern,
            "nand",
            vec![t, cp, NodeOutput(q)]
        );
        let n1 = add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(nq), cp, t]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n0), NodeOutput(q)]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n1), NodeOutput(nq)]
        );
        let mut pattern = Pattern::new(
            2,
            vec![nq, q],
            pattern
        );
        pattern.set_name(&["cp", "t"], &["nq", "q"]);
        pattern.set_description("Qn+1 = Qn xor T");
        circuit.get_patterns_mut().insert("tff".to_string(), pattern);
        
    }
    pub fn build_dff(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let (cp, d) = (PniType::Input(0), PniType::Input(1));
        let (nq, q) = (2, 3);
        let n0 = add_node(
            &mut pattern,
            "nand",
            vec![cp, d]
        );
        let n1 = add_node(
            &mut pattern,
            "nand",
            vec![cp, NodeOutput(n0)]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n0), NodeOutput(q)]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![NodeOutput(n1), NodeOutput(nq)]
        );
        let mut pattern = Pattern::new( 
            2,
            vec![nq, q],
            pattern
        );
        pattern.set_name(&["cp", "d"], &["nq", "q"]);
        pattern.set_description("Qn+1 = D");
        circuit.get_patterns_mut().insert("dff".to_string(), pattern);
        ;
    }
    pub fn build_rsff(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let (r, cp, s)=
            (PniType::Input(0), PniType::Input(1), PniType::Input(2));
        let (nq, q) = (2, 3);
        let n0 = add_node(
            &mut pattern,
            "nand",
            vec![r, cp]
        );
        let n1 = add_node(
            &mut pattern,
            "nand",
            vec![cp, s]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![PniType::NodeOutput(n1), PniType::NodeOutput(q)]
        );
        add_node(
            &mut pattern,
            "nand",
            vec![PniType::NodeOutput(nq), PniType::NodeOutput(n0)]
        );
        let mut pattern = Pattern::new(
            3,
            vec![nq, q],
            pattern
        );
        pattern.set_name(
            &["r", "cp", "s"],
            &["nq", "q"]);
        pattern.set_description("Qn+1 = S or (not(R) and Qn)");
        circuit.get_patterns_mut().insert("rsff".to_string(), pattern);
    }

    pub fn build_basic_rsff(circuit: &mut Circuit) {
        let mut pattern = Vec::new();
        let nq = add_node(
            &mut pattern,
            "nand",
            vec![PniType::Input(0), PniType::NodeOutput(1)]);
        let q = add_node(
            &mut pattern,
            "nand",
            vec![PniType::Input(1), PniType::NodeOutput(0)]);
        let mut pattern = Pattern::new(
            2,
            vec![nq, q],
            pattern,
        );
        pattern.set_name(&["rd", "sd"], &["nq", "q"]);
        circuit.get_patterns_mut().insert("brsff".to_string(), pattern);
    }
}
