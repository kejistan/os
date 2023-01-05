use crate::{print, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC_OFFSET: u8 = 32;
static PICS: spin::Mutex<ChainedPics> =
	spin::Mutex::new(unsafe { ChainedPics::new(PIC_OFFSET, PIC_OFFSET + 8) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum InterruptIdx {
	Timer = PIC_OFFSET,
}

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(double_fault_handler);
		idt.general_protection_fault
			.set_handler_fn(general_protection_fault_handler);

		idt[InterruptIdx::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

		idt
	};
}

pub fn init() {
	IDT.load();
	unsafe {
		PICS.lock().initialize();
		// TODO: enable more interrupts
		PICS.lock().write_masks(u8::MAX ^ 1, u8::MAX);
	};
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
	stack_frame: InterruptStackFrame,
	error_code: u64,
) -> ! {
	panic!(
		"EXCEPTION: DOUBLE FAULT\ncode: {}\n{:#?}",
		error_code, stack_frame
	);
}

extern "x86-interrupt" fn general_protection_fault_handler(
	stack_frame: InterruptStackFrame,
	error_code: u64,
) {
	panic!(
		"EXCEPTION: GENERAL PROTECTION FAULT\ncode: {}\n{:#?}",
		error_code, stack_frame
	);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	print!(".");
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIdx::Timer.as_u8());
	}
}

impl InterruptIdx {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}
