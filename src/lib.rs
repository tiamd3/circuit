#![allow(unused)]
mod gate;
mod signal;
mod circuit;
mod component;

use std::fmt::Debug;
use signal::BinarySignal;
use circuit::Circuit;

fn build_signal_vec<S: BinarySignal>(vec: &[u32]) -> Vec<S> {
    let mut res_vec = vec![];
    for s in vec {
        let signal = match s {
            0 => S::from_bool(false),
            1 => S::from_bool(true),
            _ => unreachable!()
        };
        res_vec.push(signal);
    }
    res_vec
}
fn test_circuit<S>(circuit: &mut Circuit<S>, input_num: usize, output_indexes: &[usize])
where
    S: BinarySignal + Default + Copy + Debug
{
    let mut input_vec = vec![S::default(); input_num];
    let n = 2u32.pow(input_num as u32);
    //print!("{}", n);
    for i in 0..n {
        let mut temp = i;
        for j in 0..input_num {
            let v= temp % 2 == 1;
            temp /= 2;
            input_vec[input_num - j - 1] = S::from_bool(v);
        }
        circuit.execute(&input_vec);
        let output = output_indexes.iter()
            .map(|i| (*i, circuit.get_signal(*i)))
            .collect::<Vec<_>>();
        println!("i: {:?}, o: {:?}", input_vec, output);
    }
}

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    use super::*;

    #[test]
    fn test() {
        let mut circuit: Circuit<Signal> = Circuit::new(2);
        let out = circuit.add_xor_gate(&[0, 1]);
        circuit.execute(
            &build_signal_vec(&[0, 1])
        );
        test_circuit(&mut circuit, 2, &[out]);
        //println!("{:#?}", circuit);
        //println!("{:?}", circuit.get_signal(out));
    }
}































