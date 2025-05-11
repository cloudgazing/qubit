use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;
use usbd_serial::SerialPort;

#[cfg(feature = "dev")]
const USB_VID: u16 = 0x16C0;
#[cfg(feature = "dev")]
const USB_PID: u16 = 0x27DD;

type SerialDevice<'a> = SerialPort<'a, UsbBus>;
type HIDDevice<'a> = HIDClass<'a, UsbBus>;
type DeviceClassList<'a> = (SerialDevice<'a>, HIDDevice<'a>);

static mut USB_ALLOC: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit();
static mut USB_DEVICE: MaybeUninit<UsbDevice<UsbBus>> = MaybeUninit::uninit();

static mut DEVICE_CLASS_LIST: MaybeUninit<DeviceClassList<'static>> = MaybeUninit::uninit();

pub fn init_usb_alloc(usb_alloc: UsbBusAllocator<UsbBus>) {
	let ptr = &raw mut USB_ALLOC;

	unsafe {
		(*ptr).write(usb_alloc);
	}
}

pub fn init_device_class_list() {
	let (serial, hid_device) = {
		let ptr = &raw mut USB_ALLOC;

		let usb_alloc = unsafe { (*ptr).assume_init_ref() };

		let serial = SerialPort::new(usb_alloc);
		let hid_class = HIDClass::new(usb_alloc, KeyboardReport::desc(), 1);

		(serial, hid_class)
	};

	let ptr = &raw mut DEVICE_CLASS_LIST;

	unsafe {
		(*ptr).write((serial, hid_device));
	}
}

pub fn init_usb_device() {
	let usb_alloc = {
		let ptr = &raw mut USB_ALLOC;

		unsafe { (*ptr).assume_init_ref() }
	};

	let vid_pid = UsbVidPid(USB_VID, USB_PID);
	let descriptors = StringDescriptors::default()
		.manufacturer("cloudgazing")
		.product("KB-RS-CLD")
		.serial_number("CLD-KB");

	let usb_device = UsbDeviceBuilder::new(usb_alloc, vid_pid)
		.strings(&[descriptors])
		.unwrap()
		.device_class(0x00)
		// .device_class(usbd_serial::USB_CLASS_CDC)
		.composite_with_iads()
		.build();

	let ptr = &raw mut USB_DEVICE;

	unsafe {
		(*ptr).write(usb_device);
	}
}

pub fn push_keyboard_events(report: KeyboardReport) {
	cortex_m::interrupt::free(|_| {
		let hid_device = {
			let ptr = &raw mut DEVICE_CLASS_LIST;

			let class_list = unsafe { (*ptr).assume_init_ref() };

			&class_list.1
		};

		_ = hid_device.push_input(&report);
	});
}

pub fn send_serial_message(msg: &'static str) {
	cortex_m::interrupt::free(|_| {
		let serial = {
			let ptr = &raw mut DEVICE_CLASS_LIST;

			let class_list = unsafe { (*ptr).assume_init_mut() };

			&mut class_list.0
		};

		_ = serial.write(msg.as_bytes());
	});
}

pub fn poll_usb_device() {
	let device = {
		let ptr = &raw mut USB_DEVICE;

		unsafe { (*ptr).assume_init_mut() }
	};

	let class_list = {
		let ptr = &raw mut DEVICE_CLASS_LIST;

		unsafe { (*ptr).assume_init_mut() }
	};

	let cls_0 = &mut class_list.0;
	let cls_1 = &mut class_list.1;

	_ = !device.poll(&mut [cls_0, cls_1]);
}
