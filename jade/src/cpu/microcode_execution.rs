use super::{
    instruction::{
        CycleType::*,
        InstructionCycle::{self, *},
    },
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
            AbsOperand1 | ReadInc => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            AbsOperand2 => {
                self.buf = self.db;
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            ImmOperand => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            ZpgOperand => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            Jsr1 => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
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
                self.ab -= 1;
                // Store lower part of ab (real stack pointer) to restore it later
                self.buf = self.ab as u8;
                self.db = self.pc as u8;
                self.write_memory(); // pc_l

                (WriteCycle, self.pc)
            }
            Jsr5 => {
                self.ab = self.pc;
                self.read_memory(); // op_h

                (ReadCycle, self.pc + 1)
            }
            Jsr6 => {
                self.ab = u16::from_le_bytes([self.sp, self.db]);
                self.sp = self.buf - 1;
                self.pc = self.ab;

                (ReadCycle, self.ab)
            }
            Sec2 => {
                self.p.set_c(true);

                (ReadCycle, self.pc)
            }
            JmpAbs => {
                self.pc = u16::from_le_bytes([self.buf, self.db]);
                self.ab = self.pc;

                (ReadCycle, self.pc)
            }
            Rts1 => {
                self.ab = u16::from_le_bytes([self.sp, 0x01]);
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Rts2 => {
                self.ab += 1;
                self.read_memory();

                self.buf = self.db;

                (ReadCycle, self.pc)
            }
            Rts3 => {
                self.ab += 1;
                self.read_memory();

                self.sp = self.ab as u8;

                (ReadCycle, self.pc)
            }
            Rts4 => {
                self.pc = u16::from_le_bytes([self.buf, self.db]);
                self.ab = self.pc;

                (ReadCycle, self.pc + 1)
            }
            Rts5 => {
                self.pc += 1;

                (ReadCycle, self.pc)
            }
            Lda => {
                self.load_a(self.db);

                (ReadCycle, self.pc + 1)
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
            Dey2 => {
                self.ab = self.pc;
                self.read_memory();

                self.on_next_cycle = Some(|cpu| {
                    cpu.load_y(cpu.y.wrapping_sub(1));
                });

                (ReadCycle, self.pc)
            }
            Adc1 => {
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
            NYI => panic!(
                "Instruction {} is not yet implemented",
                self.current_instr.unwrap().identifier
            ),
        };

        self.r = cycle_type.into();
        self.current_instr_step += 1;
        self.next_pc = next_pc;

        step
    }
}
