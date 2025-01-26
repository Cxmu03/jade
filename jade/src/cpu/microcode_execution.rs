use super::{
    instruction::{
        CycleType::*,
        InstructionCycle::{self, *},
    },
    status_flags::StatusFlags,
    Cpu, PAGE_SIZE,
};

impl Cpu {
    pub fn execute_microcode_step(&mut self) -> InstructionCycle {
        let step: InstructionCycle = self.current_instr.unwrap().cycles[self.current_instr_step];

        let (cycle_type, next_pc) = match step {
            Read => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            ReadStack => {
                self.ab = u16::from_be_bytes([0x01, self.sp]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            PopStack => {
                self.pop_stack();

                (ReadCycle, self.pc)
            }
            AbsOperand1 | ImmOperand | RelOperand | ZpgOperand | ReadInc => {
                self.ab = self.pc;
                self.read_memory();

                self.buf = self.db;

                (ReadCycle, self.pc.wrapping_add(1))
            }
            AbsXOperand => {
                let hi = self.db;
                let lo = self.buf;
                let address = u16::from_be_bytes([hi, lo]);

                let (page_crossed, new_partial_address, new_address) =
                    Self::add_offset_to_address(address, self.x);

                self.buf16 = new_address;
                self.ab = new_partial_address;
                self.read_memory();

                if !page_crossed {
                    self.skip_next_cycle();
                }

                (ReadCycle, self.pc)
            }
            AbsYOperand => {
                let hi = self.db;
                let lo = self.buf;
                let address = u16::from_be_bytes([hi, lo]);

                let (page_crossed, new_partial_address, new_address) =
                    Self::add_offset_to_address(address, self.y);

                self.buf16 = new_address;
                self.ab = new_partial_address;
                self.read_memory();

                if !page_crossed {
                    self.skip_next_cycle();
                }

                (ReadCycle, self.pc)
            }
            AbsIndexedPageCross => {
                self.ab = self.buf16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            ZpgIndexedOperand => {
                self.buf = self.db;
                self.ab = self.db as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            ZpgXOperand | IndirectXAddressLo => {
                self.ab = self.buf.wrapping_add(self.x) as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            ZpgYOperand => {
                self.ab = self.buf.wrapping_add(self.y) as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            IndirectYAddressLo => {
                self.ab = u16::from_be_bytes([0x00, self.db]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            IndirectIndexedAddressHi => {
                self.buf = self.db;
                self.ab += 1;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            AbsOperand2 => {
                self.buf = self.db;
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc.wrapping_add(1))
            }
            AbsOperand3 => {
                let lo = self.buf;
                let hi = self.db;

                self.ab = u16::from_be_bytes([hi, lo]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            RelBranch1 => {
                let operand = self.buf as i8;
                let (page_crossed, new_partial_address, new_address) =
                    Self::add_offset_to_address(self.pc, operand);

                self.pc = new_partial_address;
                self.ab = self.pc;
                self.read_memory();

                if !page_crossed {
                    self.end_instruction();
                }

                (ReadCycle, new_address)
            }
            RelBranch2 => {
                self.ab = self.pc;

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Php => {
                let mut p = self.p.clone();
                p.set_b(true);
                self.db = p.0;

                self.push_stack();

                (WriteCycle, self.pc)
            }
            Bpl => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.n() == true);

                (ReadCycle, self.pc)
            }
            Clc => {
                self.p.set_c(false);

                (ReadCycle, self.pc)
            }
            Jsr1 => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Jsr2 => {
                self.ab = 1 * PAGE_SIZE + u16::from(self.sp);
                // For some reason, the 6502 uses the stack pointer register to buffer the lower address byte,
                // which is kinda insane.
                self.sp = self.db;
                // Is basically a dummy read cycle to buffer the lower operand byte but read anyway to be
                // compatible with simulators
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Jsr3 => {
                self.db = (self.pc >> 8) as u8;
                self.write_memory(); // pc_h

                (WriteCycle, self.pc)
            }
            Jsr4 => {
                self.ab = self.ab.wrapping_sub(1);
                // Store lower part of ab (real stack pointer) to restore it later
                self.buf = self.ab as u8;
                self.db = self.pc as u8;
                self.write_memory(); // pc_l

                (WriteCycle, self.pc)
            }
            Jsr5 => {
                self.ab = self.pc;
                self.read_memory(); // op_h

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Jsr6 => {
                self.ab = u16::from_le_bytes([self.sp, self.db]);
                self.sp = self.buf - 1;
                self.pc = self.ab;

                (ReadCycle, self.ab)
            }
            Plp => {
                let mut p = StatusFlags(self.db);
                p.set_b(self.p.b());
                self.p = p;

                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Bmi => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.n() == false);

                (ReadCycle, self.pc)
            }
            Sec => {
                self.p.set_c(true);

                (ReadCycle, self.pc)
            }
            Pha => {
                self.db = self.a;
                self.push_stack();

                (WriteCycle, self.pc)
            }
            JmpAbs => {
                self.pc = u16::from_le_bytes([self.buf, self.db]);
                self.ab = self.pc;

                (ReadCycle, self.pc)
            }
            Bvc => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.v() == true);

                (ReadCycle, self.pc)
            }
            Cli => {
                self.p.set_i(false);

                (ReadCycle, self.pc)
            }
            Rts1 => {
                self.ab = u16::from_le_bytes([self.sp, 0x01]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Rts2 => {
                self.ab = self.ab.wrapping_add(1);
                self.read_memory();

                self.buf = self.db;

                (ReadCycle, self.pc)
            }
            Rts3 => {
                self.ab = self.ab.wrapping_add(1);
                self.read_memory();

                self.sp = self.ab as u8;

                (ReadCycle, self.pc)
            }
            Rts4 => {
                self.pc = u16::from_le_bytes([self.buf, self.db]);
                self.ab = self.pc;

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Rts5 => {
                self.pc = self.pc.wrapping_add(1);

                (ReadCycle, self.pc)
            }
            Pla => {
                self.a = self.db;
                self.ab = self.pc;
                self.read_memory();

                self.update_zero_negative_flags(self.a);

                (ReadCycle, self.pc)
            }
            Adc => {
                self.buf = self.db;
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    let a_before = cpu.a;
                    let operand = cpu.buf;
                    let carry = cpu.p.c() as u8;
                    let result = cpu.a.wrapping_add(operand).wrapping_add(carry);
                    let result_u16 = u16::from(cpu.a) + u16::from(operand) + u16::from(carry);

                    cpu.a = result;
                    let did_overflow = ((a_before ^ result) & (operand ^ result) & 0x80) == 0x80;

                    cpu.p.set_c(result_u16 > 0xFF);
                    cpu.p.set_v(did_overflow);
                    cpu.update_zero_negative_flags(result);
                });

                (ReadCycle, self.pc)
            }
            Bvs => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.v() == false);

                (ReadCycle, self.pc)
            }
            Sei => {
                self.p.set_i(true);

                (ReadCycle, self.pc)
            }
            Dey2 => {
                self.ab = self.pc;
                self.read_memory();

                self.on_next_cycle = Some(|cpu| {
                    cpu.load_y(cpu.y.wrapping_sub(1));
                });

                (ReadCycle, self.pc)
            }
            Txa => {
                self.load_a(self.x);

                (ReadCycle, self.pc)
            }
            Bcc => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.c() == true);

                (ReadCycle, self.pc)
            }
            Tya => {
                self.load_a(self.y);

                (ReadCycle, self.pc)
            }
            Txs => {
                self.sp = self.x;

                (ReadCycle, self.pc)
            }
            Ldy => {
                self.load_y(self.db);

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Ldx => {
                self.load_x(self.db);

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Tay => {
                self.load_y(self.a);

                (ReadCycle, self.pc)
            }
            Lda => {
                self.load_a(self.db);

                (ReadCycle, self.pc.wrapping_add(1))
            }
            Tax => {
                self.load_x(self.a);

                (ReadCycle, self.pc)
            }
            Bcs => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.c() == false);

                (ReadCycle, self.pc)
            }
            Clv => {
                self.p.set_v(false);

                (ReadCycle, self.pc)
            }
            Tsx => {
                self.load_x(self.sp);

                (ReadCycle, self.pc)
            }
            Cld => {
                self.p.set_d(false);

                (ReadCycle, self.pc)
            }
            Bne => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.z() == true);

                (ReadCycle, self.pc)
            }
            Inc2 => {
                self.ab = self.db as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Inc3 => {
                self.write_memory();

                (WriteCycle, self.pc)
            }
            Inc4 => {
                self.db = u8::wrapping_add(self.db, 1);
                self.write_memory();

                self.update_zero_negative_flags(self.db);

                (WriteCycle, self.pc)
            }
            Inx2 => {
                self.ab = self.pc;
                self.read_memory();

                // This is necessary because although the incremented x is already on the special bus, the control signal
                // to transfer sb to X (SBX or dpc3_SBX) will only fire on phi1 of the next cycle
                self.on_next_cycle = Some(|cpu| {
                    cpu.load_x(cpu.x.wrapping_add(1));
                });

                (ReadCycle, self.pc)
            }
            Beq => {
                self.ab = self.pc;
                self.read_memory();

                self.end_instruction_if(self.p.z() == false);

                (ReadCycle, self.pc)
            }
            Sed => {
                self.p.set_d(true);

                (ReadCycle, self.pc)
            }
            NYI => panic!(
                "Instruction {:02x}: {} is not yet implemented",
                self.db,
                self.current_instr.unwrap().identifier
            ),
        };

        self.r = cycle_type;
        self.current_instr_step += 1;
        self.next_pc = next_pc;

        step
    }
}
