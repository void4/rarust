use ops::*;
use ops::num_traits::FromPrimitive;
use formats::*;

/**
Run a snapshot until an exit or error occurs
*/
pub fn run(sharp: Process, gas: u64, mem: u64, debug: bool) -> Process {
    //println!("Length of binary: {0}", flat.len());
    // Process, previously serialized length, rec index
    let mut edges: Vec<u64> = vec![0];
    let mut sizes: Vec<u64> = vec![s(&sharp).len() as u64];
    let mut states: Vec<(Process)> = vec![sharp]; //d(&flat)

    let statelen = states.len() - 1 as usize;
    states[statelen].header.status = Stati::NOR as u64;
    states[statelen].header.gas = gas;
    states[statelen].header.mem = mem;

    loop {
        let mut jump_back: i64 = -2;
        let blockret = {
            let instr: u64 = 0;
            let ref mut state = states[statelen];
            //println!("{:?} {:?}", state.header.gas, state.header.ip);
            if debug {
                println!("{:?}", state.stack);
            }

            if state.header.status != Stati::NOR as u64 {
                //&& state.header.status != REC
                jump_back = (statelen as i64) - 1;
                (
                    instr.clone(),
                    Requirement {
                        length: 0,
                        stack_req: 0,
                        addtl_mem: 0,
                        gas_cost: 0,
                    },
                )
            } else if state.header.ip >= state.code.len() as u64 {
                state.header.status = Stati::OOC as u64;
                jump_back = (statelen as i64) - 1;
                (
                    instr.clone(),
                    Requirement {
                        length: 0,
                        stack_req: 0,
                        addtl_mem: 0,
                        gas_cost: 0,
                    },
                )
            } else {
                let instr = state.code[state.header.ip as usize];
                let decoded: Option<IS> = IS::from_u64(instr);
                match decoded {
                    None => {
                        state.header.status = Stati::UOC as u64;
                        jump_back = (statelen as i64) - 1;
                        (
                            instr.clone(),
                            Requirement {
                                length: 0,
                                stack_req: 0,
                                addtl_mem: 0,
                                gas_cost: 0,
                            },
                        )
                    }
                    Some(i) => {
                        let reqs = requirement(i);
                        if state.header.ip + (reqs.length as u64) - 1 >= state.code.len() as u64 {
                            state.header.status = Stati::OOA as u64;
                            jump_back = (statelen as i64) - 1;
                        }

                        if reqs.stack_req as u64 > state.stack.len() as u64 {
                            state.header.status = Stati::OOS as u64;
                            jump_back = (statelen as i64) - 1;
                        }
                        (instr, reqs.clone())
                    }
                }
            }
        };

        let instr: u64 = blockret.0;
        let reqs = blockret.1;

        fn valid_area(index: u64, process: &Process) -> bool {
            return index < process.memory.len() as u64;
        }

        if jump_back == -2 {
            for psi in (0..states.len()).rev() {
                let gascost = reqs.gas_cost as u64;
                let memcost: u64 = gascost; //XXX(ps.1 + (reqs[2] as u64))*

                if states[psi].header.mem < memcost {
                    states[psi].header.status = Stati::OOM as u64;
                    jump_back = (psi as i64) - 1;
                }

                //p.header.mem -= memcost;
                //p.header.gas -= gascost;
            }
        }

        if jump_back > -2 {
            let mut serialized: Vec<u64> = Vec::new();
            for psi in (0..states.len()).rev() {
                if psi != 0 {
                    states[statelen - psi].memory[edges[edges.len() - psi] as usize] = serialized;
                    serialized = s(&states[states.len() - psi - 1]);
                }

                if psi == 0 {
                    return states[0].clone();
                }

                if psi as i64 == jump_back - 1 {
                    break;
                }
            }
        }

        let ie: IS = unsafe { std::mem::transmute(instr as i8) };
        let ref mut state = states[statelen];
        let stacklen: usize = state.stack.len() as usize;
        if debug {
            println!(
                "INSTR: {:?} {:?}",
                IS::from_u8(instr as u8),
                state.header.ip
            );
        }
        if jump_back > -2 {
            //pass
        } else if ie == IS::RUN {
            let area = state.stack[stacklen - 3];
            let gas = state.stack[stacklen - 2];
            let mem = state.stack[stacklen - 1];

            if valid_area(area, &states[statelen]) {
                {
                    if states[statelen].header.rec == 0 {
                        let ref mut child = states[statelen];
                        child.header.rec = area + 1;

                        child.memory[area as usize][0] = Stati::NOR as u64;
                        child.memory[area as usize][2] = gas;
                        child.memory[area as usize][3] = mem;
                    }
                }
                if states[statelen].header.rec > 0
                    && states[statelen].memory[area as usize][0] == Stati::NOR as u64
                {
                    states.push(d(&states[statelen].memory[area as usize]));
                    edges.push(states[statelen].header.rec);
                    sizes.push(states[statelen].memory[area as usize].len() as u64);
                } else {
                    states[statelen].header.rec = 0;
                }
            }
        } else {
            let ref mut state = states[statelen];
            let mut jump: bool = false;
            if ie == IS::HALT {
                println!("HALT");
                state.header.status = Stati::HLT as u64;
            } else if ie == IS::RETURN {
                state.header.status = Stati::RET as u64;
                state.header.ip = 0;
                jump = true;
            } else if ie == IS::YIELD {
                state.header.status = Stati::YLD as u64;
            } else if ie == IS::JUMP {
                state.header.ip = state.stack[state.stack.len() - 1];
                state.stack.pop();
                jump = true;
            } else if ie == IS::JZ {
                if state.stack.len() >= 2 {
                    if state.stack[state.stack.len() - 2] == 0 {
                        state.header.ip = state.stack[state.stack.len() - 1];
                        jump = true;
                    }
                    state.stack.pop();
                    state.stack.pop();
                }
            } else if ie == IS::DUP {
                state.stack.push(state.stack[state.stack.len() - 1]);
            //memory effect has to be applied to all parent states!
            //check resources after decision?
            } else if ie == IS::PUSH {
                let value = state.code[(state.header.ip + 1) as usize];
                state.stack.push(value);
            } else if ie == IS::POP {
                if state.stack.len() > 0 {
                    state.stack.pop();
                }
            } else if ie == IS::DUP {
                let last = state.stack.last();
                match last {
                    Some(last) => {
                        let truelast = last.clone();
                        state.stack.push(truelast);
                    }
                    None => {}
                }
            } else if ie == IS::ADD {
                if state.stack.len() >= 2 {
                    let b = state.stack.pop().unwrap();
                    let a = state.stack.pop().unwrap();
                    state.stack.push(a.wrapping_add(b));
                }
            } else if ie == IS::SUB {
                if state.stack.len() >= 2 {
                    let b = state.stack.pop().unwrap();
                    let a = state.stack.pop().unwrap();
                    state.stack.push(a.wrapping_sub(b));
                }
            } else if ie == IS::NOT {
                if state.stack.len() >= 1 {
                    let value = state.stack.pop().unwrap();
                    if value == 0 {
                        state.stack.push(1);
                    } else {
                        state.stack.push(0);
                    }
                }
            } else if ie == IS::MEMORYLEN {
                state.stack.push(state.memory.len() as u64);
            } else if ie == IS::AREALEN {
                let area = state.stack[stacklen - 1] as usize;
                state.stack[stacklen - 1 as usize] = state.memory[area].len() as u64;
            } else if ie == IS::READ {
                if state.stack.len() >= 2 {
                    let offset = state.stack.pop().unwrap() as usize;
                    let area = state.stack.pop().unwrap() as usize;
                    if area < state.memory.len() && offset < state.memory[area].len() {
                        state.stack.push(state.memory[area][offset]);
                    }
                }
            } else if ie == IS::WRITE {
                if state.stack.len() >= 3 {
                    let value = state.stack.pop().unwrap();
                    let offset = state.stack.pop().unwrap() as usize;
                    let area = state.stack.pop().unwrap() as usize;
                    if area < state.memory.len() && offset < state.memory[area].len() {
                        state.memory[area][offset] = value;
                    }
                }
            } else if ie == IS::AREA {
                state.memory.push(Vec::with_capacity(1024)); //XXX Vec::new()
            } else if ie == IS::ALLOC {
                let size = state.stack.pop().unwrap();
                let area = state.stack.pop().unwrap() as usize;
                for _i in 0..size {
                    state.memory[area].push(0); //XXX should use resize
                }
            } else if ie == IS::DEALLOC {
                let size = state.stack.pop().unwrap();
                let area = state.stack.pop().unwrap() as usize;
                for _i in 0..size {
                    state.memory[area].pop(); //XXX should use resize
                }
            } else if ie == IS::ROT2 {
                //stack length should already be checked!
                let first = state.stack[stacklen - 1];
                let second = state.stack[stacklen - 2];
                let third = state.stack[stacklen - 3];
                state.stack[stacklen - 1] = third;
                state.stack[stacklen - 2] = first;
                state.stack[stacklen - 3] = second;
            } else {
                state.header.status = Stati::UOC as u64;
                println!("UOC{:?}", instr);
                //break;
            }

            if !jump {
                state.header.ip += reqs.length as u64;
            }

            if reqs.addtl_mem < 0 {
                /*
                let range = -reqs[2];
                for i in 0..range {
                    state.stack.pop();
                }
                */
            }
            let _stackdiff: i64 = ((states[(states.len() - 1) as usize].stack.len() as i64)
                - (stacklen as i64)) as i64;
            let stateslen = states.len();
            for i in 0..states.len() {
                states[stateslen - i - 1].header.gas -= 1; //reqs[2] as u64;
                let memcost: u64 = (sizes[states.len() - i - 1]) * (reqs.gas_cost as u64); //stackdiff
                states[stateslen - i - 1].header.mem -= memcost;
            }
        }
    }
}
