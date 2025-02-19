#![rustfmt::skip]
use lazy_static::lazy_static;
use x86_64::structures::idt;
use x86_64::registers::control::Cr2;
use crate::cpu::gdt::DOUBLE_FAULT_IST_INDEX;

lazy_static! {
    static ref IDT: idt::InterruptDescriptorTable = {
        let mut idt = idt::InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(DOUBLE_FAULT_IST_INDEX); }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        idt
    };
}

pub fn load() {
    IDT.load();
    //debug!("idt: loaded");
}

test_case!(int3_handler, {
    x86_64::instructions::interrupts::int3();
});

extern "x86-interrupt" fn divide_error_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Zero Division\n{:#?}", frame);
}

extern "x86-interrupt" fn debug_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Debug\n{:#?}", frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Non-Maskable Interrupt\n{:#?}", frame);
}

extern "x86-interrupt" fn breakpoint_handler(frame: idt::InterruptStackFrame) {
    trace!("EXCEPTION: Breakpoint\n{:#?}", frame);
}

extern "x86-interrupt" fn overflow_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Overflow\n{:#?}", frame);
}

extern "x86-interrupt" fn bound_range_exceeded_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Bound Range Exceeded\n{:#?}", frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Invalid Opcode\n{:#?}", frame);
}

extern "x86-interrupt" fn device_not_available_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Device Not Available\n{:#?}", frame);
}

extern "x86-interrupt" fn double_fault_handler(frame: idt::InterruptStackFrame, error_code: u64) -> ! {
    panic!("EXCEPTION: Double Fault with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn invalid_tss_handler(frame: idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Invalid TSS with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn segment_not_present_handler(frame: idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Segment Not Present with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(frame: idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Stack Segment Fault with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(frame: idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: General Protection Fault with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn page_fault_handler(frame: idt::InterruptStackFrame, error_code: idt::PageFaultErrorCode) {
    panic!("EXCEPTION: Page Fault with error code {:#?}\nAddress {:?}\n{:#?}", error_code, Cr2::read(), frame);
}

extern "x86-interrupt" fn x87_floating_point_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: x87 Floating Point\n{:#?}", frame);
}

extern "x86-interrupt" fn alignment_check_handler(frame: idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Alignment Check with error code {}\n{:#?}", error_code, frame);
}

extern "x86-interrupt" fn machine_check_handler(frame: idt::InterruptStackFrame) -> ! {
    panic!("EXCEPTION: Machine Check\n{:#?}", frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: SIMD Floating Point\n{:#?}", frame);
}

extern "x86-interrupt" fn virtualization_handler(frame: idt::InterruptStackFrame) {
    panic!("EXCEPTION: Virtualization\n{:#?}", frame);
}

extern "x86-interrupt" fn security_exception_handler(frame: idt::InterruptStackFrame, error_code: u64, ) {
    panic!("EXCEPTION: Security Exception with error code {}\n{:#?}", error_code, frame);
}
