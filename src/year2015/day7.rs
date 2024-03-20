use anyhow::{anyhow, Result};
use std::{borrow::Cow, collections::HashMap, fs::read_to_string, str::FromStr};

type Wire = String;

type Circuit = HashMap<Wire, Operation>;

#[derive(Debug)]
struct Instruction {
    output: Wire,
    operation: Operation,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let output = parts[1].to_string();
        if parts[0].contains("AND") {
            let operands: Vec<&str> = parts[0].split(" AND ").collect();
            let input1 = operands[0].to_string();
            let input2 = operands[1].to_string();
            return Ok(Instruction {
                output,
                operation: Operation::And(input1, input2),
            });
        } else if parts[0].contains("OR") {
            let operands: Vec<&str> = parts[0].split(" OR ").collect();
            let input1 = operands[0].to_string();
            let input2 = operands[1].to_string();
            return Ok(Instruction {
                output,
                operation: Operation::Or(input1, input2),
            });
        } else if parts[0].contains("LSHIFT") {
            let operands: Vec<&str> = parts[0].split(" LSHIFT ").collect();
            let input = operands[0].to_string();
            let shift = operands[1].parse()?;
            return Ok(Instruction {
                output: output,
                operation: Operation::LShift(input, shift),
            });
        } else if parts[0].contains("RSHIFT") {
            let operands: Vec<&str> = parts[0].split(" RSHIFT ").collect();
            let input = operands[0].to_string();
            let shift = operands[1].parse()?;
            return Ok(Instruction {
                output,
                operation: Operation::RShift(input, shift),
            });
        } else if parts[0].contains("NOT") {
            let input = parts[0].replace("NOT ", "");
            return Ok(Instruction {
                output,
                operation: Operation::Not(input),
            });
        } else {
            let input = parts[0].parse()?;
            Ok(Instruction {
                output,
                operation: Operation::Assign(input),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    And(Wire, Wire),
    Or(Wire, Wire),
    LShift(Wire, u16),
    RShift(Wire, u16),
    Not(Wire),
    Assign(Wire),
}

fn parse_input(input: &str) -> Result<Circuit> {
    let mut circuit = Circuit::new();
    for line in input.lines() {
        let instruction = line.parse::<Instruction>()?;
        circuit.insert(instruction.output, instruction.operation);
    }
    Ok(circuit)
}

fn evaluate_wire(circuit: &Circuit, wire: &Wire, cache: &mut HashMap<String, u16>) -> Result<u16> {
    let operation = circuit
        .get(wire)
        .ok_or_else(|| anyhow!("Wire not found: {}", wire))?;
    if let Some(value) = cache.get(wire) {
        return Ok(*value);
    };
    println!("Evaluating wire: {} with operation: {:?}", wire, operation);
    let result = match operation {
        Operation::Assign(value) => {
            if let Ok(value) = value.parse() {
                Ok(value)
            } else {
                evaluate_wire(circuit, value, cache)
            }
        }
        Operation::And(input1, input2) => {
            let input1 = match input1.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input1, cache)?,
            };
            let input2 = match input2.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input2, cache)?,
            };
            Ok(input1 & input2)
        }
        Operation::Or(input1, input2) => {
            let input1 = match input1.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input1, cache)?,
            };
            let input2 = match input2.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input2, cache)?,
            };
            Ok(input1 | input2)
        }
        Operation::LShift(input, shift) => {
            let input = match input.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input, cache)?,
            };
            Ok(input << shift)
        }
        Operation::RShift(input, shift) => {
            let input = match input.parse() {
                Ok(value) => value,
                Err(_) => evaluate_wire(circuit, input, cache)?,
            };
            Ok(input >> shift)
        }
        Operation::Not(input) => {
            let input = evaluate_wire(circuit, input, cache)?;
            Ok(!input)
        }
    };
    let value = result?;
    cache.insert(wire.clone(), value);
    Ok(value)
}

pub fn solve() {
    let content = read_to_string("inputs/Year2015/Day7.txt").unwrap();
    let mut circuit = parse_input(&content).unwrap();
    circuit.insert("b".to_string(), Operation::Assign("16076".to_string()));
    let mut cache = HashMap::new();
    println!(
        "Part 1: {:?}",
        evaluate_wire(&circuit, &"a".to_string(), &mut cache)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_instruction() {
        let input = "123 -> x";
        let instruction = input.parse::<Instruction>().unwrap();
        assert_eq!(instruction.output, "x");
        assert_eq!(instruction.operation, Operation::Assign("123".to_string()));

        let input = "fs AND fu -> fv";
        let instruction = input.parse::<Instruction>().unwrap();
        assert_eq!(instruction.output, "fv");
        assert_eq!(
            instruction.operation,
            Operation::And("fs".to_string(), "fu".to_string()),
            "fu"
        );

        let input = "bi LSHIFT 15 -> bm";
        let instruction = input.parse::<Instruction>().unwrap();
        assert_eq!(instruction.output, "bm");
        assert_eq!(
            instruction.operation,
            Operation::LShift("bi".to_string(), 15),
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\nd -> z\n1 AND x -> a";

        let circuit = parse_input(input).unwrap();

        assert_eq!(circuit.len(), 10);
        assert_eq!(
            circuit.get("x").unwrap(),
            &Operation::Assign("123".to_string())
        );
        assert_eq!(
            circuit.get("d").unwrap(),
            &Operation::And("x".to_string(), "y".to_string())
        );
        assert_eq!(
            circuit.get("z").unwrap(),
            &Operation::Assign("d".to_string())
        );
        assert_eq!(
            circuit.get("a").unwrap(),
            &Operation::And("1".to_string(), "x".to_string())
        );
    }

    #[test]
    fn test_evalute_wire() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\nd -> z\n1 AND x -> a";

        let circuit = parse_input(input).unwrap();
        let mut cache = HashMap::new();

        assert_eq!(
            evaluate_wire(&circuit, &"d".to_string(), &mut cache).unwrap(),
            72
        );
        assert_eq!(
            evaluate_wire(&circuit, &"z".to_string(), &mut cache).unwrap(),
            72
        );
        assert_eq!(
            evaluate_wire(&circuit, &"a".to_string(), &mut cache).unwrap(),
            1
        );
        assert_eq!(
            evaluate_wire(&circuit, &"x".to_string(), &mut cache).unwrap(),
            123
        );
        assert_eq!(
            evaluate_wire(&circuit, &"y".to_string(), &mut cache).unwrap(),
            456
        );
        assert_eq!(
            evaluate_wire(&circuit, &"h".to_string(), &mut cache).unwrap(),
            65412
        );
        assert_eq!(
            evaluate_wire(&circuit, &"i".to_string(), &mut cache).unwrap(),
            65079
        );
        assert_eq!(
            evaluate_wire(&circuit, &"f".to_string(), &mut cache).unwrap(),
            492
        );
        assert_eq!(
            evaluate_wire(&circuit, &"g".to_string(), &mut cache).unwrap(),
            114
        );
    }
}
