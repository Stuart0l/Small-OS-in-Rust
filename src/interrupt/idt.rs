use x86_64::{structures::idt::InterruptDescriptorTable, PrivilegeLevel};
use spin::Mutex;
use crate::interrupt::handler;

static IDT: Mutex<InterruptDescriptorTable> = Mutex::new({
	let idt = InterruptDescriptorTable::new();
	idt
});

/// Inserts an interrupt gate in the nth IDT entry. The Segment Selector inside the
/// gate is set to the kernel code’s Segment Selector. The Offset field is set to `addr`,
/// which is the address of the interrupt handler. The DPL field is set to 0.
macro_rules! set_intr_gate {
	($exc:ident, $fn:path) => {
		IDT.lock().$exc.set_handler_fn($fn)
			       .disable_interrupts(true);
	};
	($n:expr, $fn:path) => {
		IDT.lock()[$n].set_handler_fn($fn)
			      .disable_interrupts(true);
	};
}

/// Inserts a trap gate in the nth IDT entry. The Segment Selector inside the gate is
/// set to the kernel code’s Segment Selector. The Offset field is set to `addr`, which is
/// the address of the exception handler. The DPL field is set to 3.
macro_rules! set_system_gate {
	($exc:ident, $fn:path) => {
		IDT.lock().$exc.set_handler_fn($fn)
			       .set_privilege_level(PrivilegeLevel::Ring3);
	};
	($n:expr, $fn:path) => {
		IDT.lock()[$n].set_handler_fn($fn)
			      .set_privilege_level(PrivilegeLevel::Ring3);
	};
}

/// Inserts an interrupt gate in the nth IDT entry. The Segment Selector inside the
/// gate is set to the kernel code’s Segment Selector. The Offset field is set to `addr`,
/// which is the address of the exception handler. The DPL field is set to 3.
macro_rules! set_system_intr_gate {
	($exc:ident, $fn:path) => {
		IDT.lock().$exc.set_handler_fn($fn)
			       .disable_interrupts(true)
			       .set_privilege_level(PrivilegeLevel::Ring3);
	};
	($n:expr, $fn:path) => {
		IDT.lock()[$n].set_handler_fn($fn);
			      .disable_interrupts(true)
			      .set_privilege_level(PrivilegeLevel::Ring3);
	};
}

/// Similar to the `set_system_gate`, except the DPL field is set to 0.
macro_rules! set_trap_gate {
	($exc:ident, $fn:path) => {
		IDT.lock().$exc.set_handler_fn($fn);
	};
	($n:expr, $fn:path) => {
		IDT.lock()[$n].set_handler_fn($fn);
	};
}


pub fn setup_idt()
{
	use x86_64::instructions::tables::{lidt, DescriptorTablePointer};
	use core::mem::size_of;
	
	let ptr = DescriptorTablePointer {
		limit: (size_of::<InterruptDescriptorTable>() - 1) as u16,
		base: &*IDT.lock() as *const InterruptDescriptorTable as u64,
	};

	unsafe { lidt(&ptr) };
}


pub fn idt_set()
{
	set_system_intr_gate!(breakpoint, handler::int3);
	set_trap_gate!(double_fault, handler::doublefault_fn);
	set_intr_gate!(32, handler::timer_handler);
}
