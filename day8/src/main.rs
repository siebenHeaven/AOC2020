use std::io::{self, BufRead};
use std::str::FromStr;

mod machine {
    use std::collections::HashSet;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::fmt;
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum MachineError {
        InvalidInstruction,
        LoopDetected,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Instruction {
        opcode: String,
        operand: Vec<i64>,
    }

    pub struct Machine {
        imem: Vec<Instruction>,
        debug: bool,
        acc: i64,
        ip: i64,
        executed_set: HashSet<i64>,
    }

    impl FromStr for Instruction {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.split(' ').collect();
            let opcode = coords[0].to_string();
            let operand = vec![coords[1].parse::<i64>()?];
            Ok(Instruction {
                opcode: opcode,
                operand: operand,
            })
        }
    }

    impl fmt::Display for Instruction {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(&self.opcode)?;
            fmt.write_str(" ")?;
            fmt.write_str(&self.operand[0].to_string())?;
            Ok(())
        }
    }

    impl Instruction {
        pub fn get_operand(&self, i: usize) -> i64 {
            self.operand[i]
        }
    }

    impl Machine {
        pub fn new(imem: Vec<Instruction>, debug: bool) -> Self {
            Self {
                imem,
                debug,
                acc: 0,
                ip: 0,
                executed_set: HashSet::new(),
            }
        }
        fn execute(&mut self, ins: &Instruction) -> std::result::Result<(), MachineError> {
            match ins.opcode.as_str() {
                "acc" => {
                    self.acc += ins.operand[0];
                    Ok(())
                }
                "jmp" => {
                    self.ip += ins.operand[0] - 1; // ip += 1 happens as part of run hence compensate via -1
                    Ok(())
                }
                "nop" => Ok(()),
                _ => Err(MachineError::InvalidInstruction),
            }
        }
        pub fn run(&mut self) -> std::result::Result<(), MachineError> {
            while self.ip < self.imem.len().try_into().unwrap() {
                let ins = self.imem[usize::try_from(self.ip).unwrap()].clone();
                if self.debug {
                    println!("{:#?}, acc:{}, ip:{}", ins, self.acc, self.ip);
                }
                if self.executed_set.contains(&self.ip) {
                    return Err(MachineError::LoopDetected);
                } else {
                    self.executed_set.insert(self.ip);
                }
                self.execute(&ins)?;
                self.ip += 1;
            }
            println!("acc:{}, ip:{}", self.acc, self.ip);
            Ok(())
        }
    }
}

fn main() -> std::io::Result<()> {
    let instructions: Vec<machine::Instruction> = io::stdin()
        .lock()
        .lines()
        .map(|line| machine::Instruction::from_str(&line.unwrap()).unwrap())
        .collect();

    // Check if substituting jmp works
    let jmp_ids = instructions.iter().enumerate().filter_map(|ins_tuple| {
        let (id, ins) = ins_tuple;
        if ins.to_string().starts_with("jmp") {
            Some(id)
        } else {
            None
        }
    });

    for jmp_id in jmp_ids {
        println!("Trying jump_id:{}", jmp_id);

        let mut instructions_cloned = instructions.clone();
        let nop_ins = machine::Instruction::from_str("nop 0").unwrap();
        instructions_cloned[jmp_id] = nop_ins;
        let mut m = machine::Machine::new(instructions_cloned, false);
        match m.run() {
            Err(machine::MachineError::LoopDetected) => continue,
            Ok(()) => return Ok(()),
            _ => unreachable!(),
        }
    }

    // Check if substituting nop works
    let nop_ids = instructions.iter().enumerate().filter_map(|ins_tuple| {
        let (id, ins) = ins_tuple;
        if ins.to_string().starts_with("nop") {
            Some(id)
        } else {
            None
        }
    });

    for nop_id in nop_ids {
        println!("Trying nop_id:{}", nop_id);

        let mut instructions_cloned = instructions.clone();
        let addr = instructions_cloned[nop_id].get_operand(0);
        let jmp_ins =
            machine::Instruction::from_str(format!("{} {}", "jmp ", addr).as_str()).unwrap();
        instructions_cloned[nop_id] = jmp_ins;
        let mut m = machine::Machine::new(instructions_cloned, false);
        match m.run() {
            Err(machine::MachineError::LoopDetected) => continue,
            Ok(()) => return Ok(()),
            _ => unreachable!(),
        }
    }

    unreachable!();
}
