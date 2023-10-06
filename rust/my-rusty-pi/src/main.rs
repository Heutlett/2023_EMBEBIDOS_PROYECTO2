#![no_std]
#![no_main]

// Direcciones de registros para UART1 en BCM2837
const UART_BASE: u32 = 0x3F20_1000;
const UART_DR: *mut u32 = (UART_BASE + 0x00) as *mut u32;
const UART_FR: *mut u32 = (UART_BASE + 0x18) as *mut u32;

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3F20_001C;
const GPIO_CLR0: u32 = 0x3F20_0028;

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;
    
   global_asm!(
        ".section .text._start"
    );
}


struct GPIO;
impl GPIO {
    pub fn set_output (pin: u32){
        
        let reg      = pin/10;
        let register = match reg {
            0 => GPIO_FSEL0, 
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Fatal error while setting output"),
        };

        let mut val: u32 = 0;
        unsafe {
          
            val = core::ptr::read_volatile(register as *mut u32);
        }
        // is our data able to be put in this reg? check pag 93 bcm2835 datasheet

        // create mask
        let mut mask: u32 = 0b111;

        //shift mask to the right location
        let pinnum = pin%10;
        mask = mask << pinnum * 3;

        //and in the NOT of the mask, check if all our *val* bits are cleared
        val = val & !(mask);

        //set our value
        val |= 1 << pinnum *3;
        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
        
    }

    pub fn set(pin: u32){
        let bitpos = pin;

        let mut val: u32  = 0;

        unsafe {
          
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }
        // shift to get to the right bits
        val |= 1 << bitpos;

        unsafe {
          
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }


    }

    pub fn clear(pin: u32){
        let bitpos = pin;

        let mut val: u32  = 0;

        unsafe {
          
            val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
        }
        // shift to get to the right bits
        val |= 1 << bitpos;

        unsafe {
          
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
        }
    }

    pub fn uart_init() {
        // Configurar la velocidad de baudios (por ejemplo, 115200 bps)
        let baudrate = 115200;
        let baudrate_divisor = 250_000_000 / (8 * baudrate);
    
        // Configurar el divisor de baudios
        unsafe {
            core::ptr::write_volatile(UART_DR, 0); // Para asegurar que el registro DR esté limpio
            core::ptr::write_volatile(UART_FR, 0x30); // Habilitar FIFO y limpiar las colas de datos
            core::ptr::write_volatile(UART_DR, baudrate_divisor);
            core::ptr::write_volatile(UART_FR, 0); // Deshabilitar FIFO
        }
    }
    
    pub fn uart_read_byte() -> u8 {
        // Esperar hasta que haya datos disponibles
        unsafe {
            while core::ptr::read_volatile(UART_FR) & 0x10 != 0 {}
            core::ptr::read_volatile(UART_DR) as u8
        }
    }
    
    fn process_uart_data(received_byte: u8) {
        // Extraer los primeros 4 bits del byte recibido
        let first_4_bits = received_byte & 0xF;
    
        // Pines GPIO a utilizar
        let gpio_pins = [22, 23, 24, 27];
    
        // Configurar los pines GPIO como salidas
        for &pin in gpio_pins.iter() {
            GPIO::set_output(pin);
        }
    
        // Escribir los valores de los pines GPIO según los primeros 4 bits
        for (index, &pin) in gpio_pins.iter().enumerate() {
            let bit_value = (first_4_bits >> index) & 0x1;
            
            if bit_value == 1 {
                GPIO::set(pin);
            } else {
                GPIO::clear(pin);
            }
        }
    }
    
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> !{

    //GPIO::uart_init();
    GPIO::set_output(14);

    loop {

        //turn pin ON
        GPIO::set(14);

        for _ in 1..50000 {
            unsafe {asm!("nop");}
        }

        //turn pin off
        GPIO::clear(14);

        for _ in 1..50000 {
            unsafe {asm!("nop");}
        }

        // // Leer un byte del UART
        // let received_byte = GPIO::uart_read_byte();

        // // Procesar el byte recibido según sea necesario
        // GPIO::process_uart_data(received_byte);


    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
