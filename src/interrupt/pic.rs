use spin::Mutex;
use x86_64::instructions::port::Port;

const PIC_1_CMD: u16 = 0x0020;
const PIC_1_DATA: u16 = 0x0021;
const PIC_2_CMD: u16 = 0x00a0;
const PIC_2_DATA: u16 = 0x00a1;
const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

const ICW1_ICW4: u8 = 0x01;		// ICW4 (not) needed
const ICW1_SINGLE: u8 = 0x02;		// Single (cascade) mode
const ICW1_INTERVAL4: u8 = 0x04;	// Call address interval 4 (8)
const ICW1_LEVEL: u8 = 0x08;		// Level triggered (edge) mode
const ICW1_INIT: u8 = 0x10;		// Initialization - required!
 
const ICW4_8086: u8 = 0x01;		// 8086/88 (MCS-80/85) mode
const ICW4_AUTO: u8 = 0x02;		// Auto (normal) EOI
const ICW4_BUF_SLAVE: u8 = 0x08;	// Buffered mode/slave
const ICW4_BUF_MASTER: u8 = 0x0C;	// Buffered mode/master
const ICW4_SFNM: u8 = 0x10;		// Special fully nested (not)

const PIC_EOI: u8 = 0x20;		// End-of-interrupt command code

#[derive(Clone)]
struct ProgrammableInterruptController {
	pic_1_data: Port<u8>,
	pic_1_cmd: Port<u8>,
	pic_2_data: Port<u8>,
	pic_2_cmd: Port<u8>,
}

static PIC: Mutex<ProgrammableInterruptController> =
	Mutex::new({
		let pic = ProgrammableInterruptController::new();
		pic
	});

impl ProgrammableInterruptController {

	pub const fn new() -> ProgrammableInterruptController
	{
		ProgrammableInterruptController {
			pic_1_data: Port::new(PIC_1_DATA),
			pic_1_cmd: Port::new(PIC_1_CMD),
			pic_2_data: Port::new(PIC_2_DATA),
			pic_2_cmd: Port::new(PIC_2_CMD),
		}
	}

	pub unsafe fn init(&mut self)
	{
		let mut wait_port: Port<u8> = Port::new(0x80);
		let mut io_wait = || wait_port.write(0);

		let a1 = self.pic_1_data.read();
		let a2 = self.pic_2_data.read();	// save masks

		self.pic_1_cmd.write(ICW1_INIT | ICW1_ICW4);
		io_wait();
		self.pic_2_cmd.write(ICW1_INIT | ICW1_ICW4);		// starts the initialization sequence (in cascade mode)
		io_wait();
		self.pic_1_data.write(PIC_1_OFFSET);			// ICW2: Master PIC vector offset
		io_wait();
		self.pic_2_data.write(PIC_2_OFFSET);			// ICW2: Slave PIC vector offset
		io_wait();
		self.pic_1_data.write(4);				// ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
		io_wait();
		self.pic_2_data.write(2);				// ICW3: tell Slave PIC its cascade identity (0000 0010)
		io_wait();

		self.pic_1_data.write(ICW4_8086);
		io_wait();
		self.pic_2_data.write(ICW4_8086);
		io_wait();

		self.pic_1_data.write(a1);
		self.pic_2_data.write(a2);	// restore saved masks.
	}

	pub unsafe fn send_eoi(&mut self, irq: u8)
	{
		if irq >= 8 {
			self.pic_2_cmd.write(PIC_EOI);
		}

		self.pic_1_cmd.write(PIC_EOI);
	}
}

pub fn init_pic()
{
	unsafe { PIC.lock().init(); }
}

pub fn notify_eoi(irq: u8)
{
	unsafe { PIC.lock().send_eoi(irq); }
}
