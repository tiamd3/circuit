
#[derive(Debug)]
pub struct TruthTable {
    data: Vec<Vec<usize>>,
    input_size: usize,
    output_size: usize,
}

impl TruthTable {
    pub fn new(
        data: Vec<Vec<usize>>,
        input_size: usize,
        output_size: usize) -> TruthTable {
        Self { data, input_size, output_size }
    }

    pub fn input_size(&self) -> usize { self.input_size }
    pub fn output_size(&self) -> usize { self.output_size }
    pub fn prepare_print(input_size: usize, output_size: usize){
        let n = 2u32.pow(input_size as u32) as usize;
        let mut data = vec![vec![]; n];

        for i in 0..n {
            let mut temp = i;
            let mut input_line = vec![None; input_size + output_size];
            for j in 0..input_size {
                let value = temp % 2;
                temp /= 2;
                input_line[input_size - j - 1] = Some(value);
            }
            data[i] = input_line;
        }

        Self::print(&data);
    }

    pub fn print(data: &Vec<Vec<Option<usize>>>) {
        println!("vec![");
        for line in data {
            print!("vec![");
            for value in line {
                Self::print_value(value);
                print!(", ")
            }
            println!("],");
        }
        println!("]");
    }

    pub fn print_value(value: &Option<usize>) {
        if let Some(v) = value {
            print!("{}", v);
        } else {
            print!("_");
        }
    }
}

pub struct KarnaughMap {
    data: Vec<Vec<usize>>,
}

impl KarnaughMap {
    pub fn new(truth_table: &TruthTable) -> Result<Vec<KarnaughMap>, String> {
        let input_size = truth_table.input_size();
        if input_size < 3 { return Err(String::from("Input size must be at least 3")); }
        if input_size == 3 {
            todo!()
        }
        if input_size == 4 {
            todo!()
        }
        if input_size == 5 {
            todo!()
        }
        Ok(vec![])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        TruthTable::prepare_print(2, 2);

        let tt = TruthTable::new(
            vec![
                vec![0, 0, 0, 0, 1, ],
                vec![0, 0, 1, 1, 1, ],
                vec![0, 1, 0, 1, 0, ],
                vec![0, 1, 1, 1, 1, ],
                vec![1, 0, 0, 0, 1, ],
                vec![1, 0, 1, 1, 0, ],
                vec![1, 1, 0, 1, 0, ],
                vec![1, 1, 1, 0, 1, ],
            ],
            3,
            2
        );

        let tt = TruthTable::new(
            vec![
                vec![0, 0, 0, 1, ],
                vec![0, 1, 1, 0, ],
                vec![1, 0, 1, 0, ],
                vec![1, 1, 1, 0, ],
            ],
            2,
            2
        );
    }
}
