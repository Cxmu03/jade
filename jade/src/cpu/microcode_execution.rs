#![allow(clippy::bool_comparison)]

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

        // TODO: Reorder match arms
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
            DummyWrite => {
                self.write_memory();

                (WriteCycle, self.pc)
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
            AbsXOperand => self.process_indexed_operand::<true>(self.x),
            AbsXOperandNoSkip => self.process_indexed_operand::<false>(self.x),
            AbsYOperand => self.process_indexed_operand::<true>(self.y),
            AbsYOperandNoSkip => self.process_indexed_operand::<false>(self.x),
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
            ZpgOperand2 | IndirectYAddressLo => {
                self.ab = self.db as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            IndirectIndexedAddressHi => {
                self.buf = self.db;
                self.ab += 1;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            IndirectOperand => {
                self.ab = u16::from_be_bytes([self.db, self.buf]);
                self.read_memory();
                self.buf = self.db;

                (ReadCycle, self.pc)
            }
            AbsOperand2 => {
                self.buf = self.db;
                self.ab = self.ab.wrapping_add(1);
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
            PullStatus => {
                self.ab = Self::add_offset_to_stack_address(self.ab, 1);
                self.read_memory();
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.p.0 = cpu.db;
                });

                (ReadCycle, self.pc)
            }
            And => {
                self.buf = self.db;

                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.a &= cpu.buf;
                    cpu.update_zero_negative_flags(cpu.a);
                });

                (ReadCycle, self.pc)
            }
            AslA => {
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.p.set_c(cpu.a & 0x80 > 0);
                    cpu.a <<= 1;
                    cpu.update_zero_negative_flags(cpu.a);
                });

                (ReadCycle, self.pc)
            }
            Asl => {
                self.p.set_c(self.db & 80 > 0);
                self.db <<= 1;
                self.write_memory();
                self.update_zero_negative_flags(self.db);

                (WriteCycle, self.pc)
            }
            Eor => {
                self.buf = self.db;

                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.a ^= cpu.buf;
                    cpu.update_zero_negative_flags(cpu.a);
                });

                (ReadCycle, self.pc)
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
            RorA => {
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    let new_carry = cpu.a & 1 == 1;
                    cpu.a = (cpu.a >> 1) | ((cpu.p.c() as u8) << 7);
                    cpu.update_zero_negative_flags(cpu.a);
                    cpu.p.set_c(new_carry);
                });

                (ReadCycle, self.pc)
            }
            Ror => {
                let new_carry = self.db & 1 == 1;
                self.db = (self.db >> 1) | ((self.p.c() as u8) << 7);
                self.update_zero_negative_flags(self.db);
                self.p.set_c(new_carry);
                self.write_memory();

                (WriteCycle, self.pc)
            }
            RolA => {
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    let new_carry: bool = cpu.a & 0x80 == 0x80;
                    cpu.a = (cpu.a << 1) | (cpu.p.c() as u8);
                    cpu.update_zero_negative_flags(cpu.a);
                    cpu.p.set_c(new_carry);
                });

                (ReadCycle, self.pc)
            }
            Rol => {
                let new_carry = self.db & 0x80 == 0x80;
                self.db = (self.db << 1) | (self.p.c() as u8);
                self.update_zero_negative_flags(self.db);
                self.p.set_c(new_carry);
                self.write_memory();

                (WriteCycle, self.pc)
            }
            Rts1 => {
                self.ab = u16::from_le_bytes([self.sp, 0x01]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Rts2 => {
                self.ab = Self::add_offset_to_stack_address(self.ab, 1);
                self.read_memory();

                self.buf = self.db;

                (ReadCycle, self.pc)
            }
            Rts3 => {
                self.ab = Self::add_offset_to_stack_address(self.ab, 1);
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
                    cpu.a = cpu.add_with_carry::<true>(cpu.a, cpu.buf, cpu.p.c());
                });

                (ReadCycle, self.pc)
            }
            Sbc => {
                self.buf = !self.db;

                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.a = cpu.add_with_carry::<true>(cpu.a, cpu.buf, cpu.p.c());
                });

                (ReadCycle, self.pc)
            }
            Cmp => {
                self.buf = !self.db;

                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.a = cpu.add_with_carry::<false>(cpu.a, cpu.buf, true);
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
            Dey => {
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
            LsrA => {
                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.p.set_c(cpu.a & 1 == 1);
                    cpu.a >>= 1;
                    cpu.update_zero_negative_flags(cpu.a);
                });

                (ReadCycle, self.pc)
            }
            Lsr => {
                self.p.set_c(self.db & 1 == 1);
                self.db >>= 1;
                self.write_memory();
                self.update_zero_negative_flags(self.db);

                (WriteCycle, self.pc)
            }
            Ora => {
                self.buf = self.db;

                self.on_next_cycle = Some(|cpu: &mut Cpu| {
                    cpu.a |= cpu.buf;
                    cpu.update_zero_negative_flags(cpu.a);
                });

                (ReadCycle, self.pc)
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
            Inc => {
                self.db = u8::wrapping_add(self.db, 1);
                self.write_memory();

                self.update_zero_negative_flags(self.db);

                (WriteCycle, self.pc)
            }
            Dec => {
                self.db = u8::wrapping_sub(self.db, 1);
                self.write_memory();

                self.update_zero_negative_flags(self.db);

                (WriteCycle, self.pc)
            }
            Inx => {
                self.ab = self.pc;
                self.read_memory();

                // This is necessary because although the incremented x is already on the special bus, the control signal
                // to transfer sb to X (SBX or dpc3_SBX) will only fire on phi1 of the next cycle
                self.on_next_cycle = Some(|cpu| {
                    cpu.load_x(cpu.x.wrapping_add(1));
                });

                (ReadCycle, self.pc)
            }
            Iny => {
                self.ab = self.pc;
                self.read_memory();

                self.on_next_cycle = Some(|cpu| {
                    cpu.load_x(cpu.y.wrapping_add(1));
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
