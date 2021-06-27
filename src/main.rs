use clap::Clap;
use std::fs::File;
use std::result::Result;
use std::io;
use std::io::Read;

const TAPELEN: usize = 0xffff;

#[derive(Clap, Debug)]
struct Args {
    #[clap(short, long, about = "Path to the bf file")]
    path: String,
    #[clap(short, long, about = "Generate debug files")]
    debug: bool,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum OpCode {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Out,
    Inp,
    LoopS,
    LoopE,
}

#[derive(Debug)]
struct Operation {
    op_code: OpCode,
    times: u16
}

fn read_source(path: String) -> Result<String, io::Error> {
    let mut fd = File::open(path)?;
    let mut s = String::new();
    fd.read_to_string(&mut s)?;
    Ok(s)
}

fn lex(source: &String) -> Result<Vec<OpCode>, io::Error> {
    let mut op_codes: Vec<OpCode> = Vec::with_capacity(source.len());
    for c in source.chars() {
        match c {
            '>' => op_codes.push(OpCode::IncPtr),
            '<' => op_codes.push(OpCode::DecPtr),
            '+' => op_codes.push(OpCode::IncVal),
            '-' => op_codes.push(OpCode::DecVal),
            '.' => op_codes.push(OpCode::Out),
            ',' => op_codes.push(OpCode::Inp),
            '[' => op_codes.push(OpCode::LoopS),
            ']' => op_codes.push(OpCode::LoopE),
            _ => ()
        }
    }
    Ok(op_codes)
}

fn parse(inp: &Vec<OpCode>) -> Result<Vec<Operation>, io::Error> {
    let mut op_codes: Vec<Operation> = Vec::with_capacity(inp.len());
    let mut count: u16 = 1;
    for n in 0..inp.len()-1 {
        if inp[n]==inp[n+1] {
            count = count+1;
        } else {
            op_codes.push(Operation{
                op_code:inp[n],
                times:count 
            });
            count = 1;
        }
    }
    op_codes.push(Operation{
        op_code:inp[inp.len()-1],
        times:count
    });
    Ok(op_codes)
}

fn interpret(operations: &Vec<Operation>) -> Result<(), io::Error> {
    let mut ptr: usize = 0;
    let mut op_ptr: usize = 0;
    let mut loop_stack = Vec::new(); 
    let mut tape = vec![0u8; TAPELEN];
    while op_ptr!=operations.len()-1 {
        let curr_operation = &operations[op_ptr];
        match curr_operation.op_code {
            OpCode::IncPtr => {
                ptr = ptr + curr_operation.times as usize;
            },
            OpCode::DecPtr => {
                ptr = ptr - curr_operation.times as usize;
            },
            OpCode::IncVal => {
                tape[ptr] = tape[ptr] + curr_operation.times as u8;
            },
            OpCode::DecVal => {
                tape[ptr] = tape[ptr] - curr_operation.times as u8;
            },
            OpCode::Out => {
                for _ in 0..curr_operation.times {
                    print!("{}", tape[ptr] as char)
                }
            },
            OpCode::Inp => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let n: u8 = input.trim().parse().unwrap();
                tape[ptr] = n;
            },
            OpCode::LoopS => {
                loop_stack.push(op_ptr);
            },
            OpCode::LoopE => {
                if tape[ptr]!=0{
                    op_ptr = loop_stack[loop_stack.len()-1];
                } else {
                    loop_stack.pop();
                }
            }
        }
        op_ptr = op_ptr + 1;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Args = Args::parse();
    let source = read_source(args.path)?;
    let opcodes = lex(&source).expect("failed to lex");
    let operations = parse(&opcodes).expect("failed to parse");
    interpret(&operations)?;
    Ok(())
}
