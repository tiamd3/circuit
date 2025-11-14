use digicir::circuit::Circuit;

fn try_create_ff() {
   let mut circuit = Circuit::new(2);
   let inputs = circuit.get_input();
   let outputs = circuit.advance_output_for_flipflop();
   let (q, nq) = (outputs[0], outputs[1]);
   let (cp, d) = (inputs[0], inputs[1]);
   
   let o0 = circuit.add_gate("nand", &[cp, d]);
   let o1 = circuit.add_gate("nand",&[o0, cp]);
   
   circuit.execute_sequential_mut(&[0, 1]);
   
}
fn main() {
   
   println!("hello world");
}