extern crate num_derive;
pub extern crate num_traits;

#[derive(Debug, PartialEq, num_derive::FromPrimitive)]
pub enum IS {
    HALT,
    RETURN,
    YIELD,
    RUN,
    JUMP,
    JZ,
    PUSH,
    POP,
    DUP,
    FLIP,
    KEYSET,
    KEYHAS,
    KEYGET,
    KEYDEL,
    STACKLEN,
    MEMORYLEN,
    AREALEN,
    READ,
    WRITE,
    AREA,
    DEAREA,
    ALLOC,
    DEALLOC,
    ADD,
    SUB,
    NOT,
    MUL,
    DIV,
    MOD,
    SHA256,
    ECVERIFY,
    ROT,
    ROT2,
}

#[derive(Clone)]
pub struct Requirement {
    pub length: i32,
    pub stack_req: i32,
    pub addtl_mem: i32,
    pub gas_cost: i32,
}

//Instruction length, stack reqs, additional memory, gas cost
pub fn requirement(is: IS) -> Requirement {
    match is {
        IS::HALT => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::RETURN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::YIELD => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::RUN => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: -3,
            gas_cost: 0,
        },
        IS::JUMP => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 1,
        },
        IS::JZ => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 1,
        },
        IS::PUSH => Requirement {
            length: 2,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        },
        IS::POP => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: -1,
            gas_cost: 2,
        }, //XXX changed stack effect
        IS::DUP => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 4,
        },
        IS::FLIP => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::KEYSET => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        }, //keys
        IS::KEYHAS => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::KEYGET => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 6,
        },
        IS::KEYDEL => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 4,
        },
        IS::STACKLEN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        }, //lens
        IS::MEMORYLEN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        },
        IS::AREALEN => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 2,
        },
        IS::READ => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 2,
        }, //r/w
        IS::WRITE => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: -3,
            gas_cost: 2,
        },
        IS::AREA => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 10,
        }, //a/d
        IS::DEAREA => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 10,
        }, //use after free!
        IS::ALLOC => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        }, //alloc/dealloc
        IS::DEALLOC => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        },
        IS::ADD => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 6,
        },
        IS::SUB => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 6,
        },
        IS::NOT => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::MUL => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 8,
        },
        IS::DIV => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 10,
        },
        IS::MOD => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 10,
        },
        IS::SHA256 => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 100,
        },
        IS::ECVERIFY => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 100,
        },
        IS::ROT => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: 0,
            gas_cost: 10,
        },
        IS::ROT2 => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: 0,
            gas_cost: 10,
        },
    }
}
