use super::{Cpu, PAGE_SIZE, instruction::{InstructionCycle::{self, *}, CycleType::*}};

impl Cpu {
    pub fn execute_microcode_step(&mut self) -> InstructionCycle {
        let step: InstructionCycle = self.current_instr.unwrap().cycles[self.current_instr_step];

        let (cycle_type, next_pc) = match step {
            Read => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc)
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
                self.sp = self.buf;
                self.pc = self.ab;

                (ReadCycle, self.ab)
            }
            Sec2 => {
                self.p.set_c(true);

                (ReadCycle, self.pc)
            }
            Lda => {
                self.a = self.db;

                (ReadCycle, self.pc + 1)
            }
            Inx2 => {
                self.ab = self.pc;
                self.read_memory();

                // This is necessary because although the incremented x is already on the special bus, the control signal
                // to transfer sb to X (SBX or dpc3_SBX) will only fire on phi1 of the next cycle
                self.on_next_cycle = Some(|cpu| {
                    cpu.x = cpu.x.wrapping_add(1);
                });

                (ReadCycle, self.pc)
            }
            Dey2 => {
                self.ab = self.pc;
                self.read_memory();

                self.on_next_cycle = Some(|cpu| {
                    cpu.y = cpu.y.wrapping_sub(1);
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