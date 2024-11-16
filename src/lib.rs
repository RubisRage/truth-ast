use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum LogicOp {
    And,
    Or,
    Not,
    Driver,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Instance {
    op: LogicOp,
    output: String,
    inputs: Vec<String>,
}

impl Instance {
    fn new(op: LogicOp, output: &str, inputs: &[&str]) -> Self {
        let inputs = Vec::from(inputs);

        Self {
            op,
            output: output.to_string(),
            inputs: inputs.into_iter().map(String::from).collect(),
        }
    }
}

#[allow(dead_code)]
const EXAMPLE: &str = "  	
    // Not gates
    not ng1(ncin, cin);
    not ng2(na0, a0);
    not ng3(nb0, b0);

    // Compute out
    and g5(f5, cin, na0, nb0);
    and g6(f6, ncin, na0, b0);
    and g7(f7, cin, a0, b0);
    and	g8(f8, ncin, a0, nb0);
    or	g9(out, f5, f6, f7, f8);

    // Compute cout
    and g1(f1, a0, b0);
    and g2(f2, cin, a0);
    and g3(f3, cin, b0);
    or	g4(cout, f1, f2, f3);
";

#[derive(Debug)]
struct StructuralAST {
    name: String,
    op: LogicOp,
    children: Vec<StructuralAST>,
}

fn build_truth_table(
    output: String,
    inputs: Vec<String>,
    instances: &[Instance],
) -> HashMap<Vec<String>, u8> {
    let map = HashMap::<String, Instance>::from_iter(
        instances.iter().cloned().map(|i| (i.output.clone(), i)),
    );

    let target = map.get(&output).expect("Output instance should be present");
    let mut root = StructuralAST {
        name: target.output.to_string(),
        op: target.op,
        children: vec![],
    };

    let mut stack = VecDeque::from([&mut root]);

    while let Some(current) = stack.pop_front() {
        let instance = map.get(&current.name).unwrap();

        for input in &instance.inputs[..] {
            let child = StructuralAST {
                name: input.to_string(),
                op: map.get(input).unwrap().op,
                children: vec![],
            };

            current.children.push(child);
        }

        for child in current.children.iter_mut() {
            stack.push_back(child);
        }
    }

    dbg!(root);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let instances = [
            Instance::new(LogicOp::Driver, "cin", &[]),
            Instance::new(LogicOp::Driver, "a0", &[]),
            Instance::new(LogicOp::Driver, "b0", &[]),
            Instance::new(LogicOp::Not, "ncin", &["cin"]),
            Instance::new(LogicOp::Not, "na0", &["a0"]),
            Instance::new(LogicOp::Not, "nb0", &["b0"]),
            Instance::new(LogicOp::And, "f5", &["cin", "na0", "nb0"]),
            Instance::new(LogicOp::And, "f6", &["ncin", "na0", "b0"]),
            Instance::new(LogicOp::And, "f7", &["cin", "a0", "b0"]),
            Instance::new(LogicOp::And, "f8", &["ncin", "a0", "nb0"]),
            Instance::new(LogicOp::Or, "out", &["f5", "f6", "f7", "f8"]),
        ];

        build_truth_table(
            "out".to_string(),
            vec!["cin".to_string(), "a0".to_string(), "b0".to_string()],
            &instances[..],
        );
    }
}
