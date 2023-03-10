use lazy_static::lazy_static;
use x86_64::registers::segmentation::{Segment, DS, ES, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub fn init() {
	use x86_64::instructions::segmentation::CS;
	use x86_64::instructions::tables::load_tss;

	GDT.gdt.load();
	unsafe {
		CS::set_reg(GDT.selectors.code_selector);
		SS::set_reg(GDT.selectors.data_selector);
		ES::set_reg(GDT.selectors.data_selector);
		DS::set_reg(GDT.selectors.data_selector);
		load_tss(GDT.selectors.tss_selector);
	}
}

const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096 * 5;
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			// TODO
			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			let stack_end = stack_start + STACK_SIZE;
			stack_end
		};
		tss
	};
}

lazy_static! {
	static ref GDT: GDTInfo = {
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
		let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
		GDTInfo {
			gdt,
			selectors: Selectors {
				code_selector,
				tss_selector,
				data_selector,
			},
		}
	};
}

struct GDTInfo {
	gdt: GlobalDescriptorTable,
	selectors: Selectors,
}

struct Selectors {
	code_selector: SegmentSelector,
	tss_selector: SegmentSelector,
	data_selector: SegmentSelector,
}
