extern crate kvm_bindings;
extern crate kvm_ioctls;

use std::io::Write;
use std::ptr::null_mut;
use std::slice;

use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::{Error, Kvm, VcpuExit};

fn main() -> Result<(), Error> {
    // One page of memory (4k) for the VM.
    let mem_size = 0x4000;
    let guest_addr = 0x1000;
    let asm_code: &[u8] = &[
        0xba, 0xf8, 0x03, /* mov $0x3f8, %dx */
        0x00, 0xd8, /* add %bl, %al */
        0x04, b'0', /* add $'0', %al */
        0xee, /* out %al, %dx */
        0xf4, /* hlt */
    ];

    // 1. Instantiate KVM.
    let kvm = Kvm::new()?;

    // 2. Create a VM.
    let vm = kvm.create_vm()?;

    // 3. Initialize Guest Memory.
    let load_addr: *mut u8 = unsafe {
        libc::mmap(
            null_mut(),
            mem_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_SHARED | libc::MAP_NORESERVE,
            -1,
            0,
        ) as *mut u8
    };

    let slot = 0;
    let mem_region = kvm_userspace_memory_region {
        slot,
        guest_phys_addr: guest_addr,
        memory_size: mem_size as u64,
        userspace_addr: load_addr as u64,
        flags: 0,
    };
    unsafe { vm.set_user_memory_region(mem_region)? };

    // Write the code to the VM memory.
    unsafe {
        let mut slice = slice::from_raw_parts_mut(load_addr, mem_size);
        slice.write_all(&asm_code)?;
    }

    // 4. Create one vCPU.
    // This is where the VMCS is created.
    let vcpu_fd = vm.create_vcpu(0)?;

    // 5. Initialize special registers.
    let mut vcpu_sregs = vcpu_fd.get_sregs()?;
    vcpu_sregs.cs.base = 0;
    vcpu_sregs.cs.selector = 0;
    vcpu_fd.set_sregs(&vcpu_sregs)?;

    // 6. Initial general purpose registers
    // RIP is set to the code's guest address
    // AX is set to 0, BX is set to 4.
    let mut vcpu_regs = vcpu_fd.get_regs()?;
    vcpu_regs.rip = guest_addr;
    vcpu_regs.rax = 0;
    vcpu_regs.rbx = 4;
    vcpu_regs.rflags = 2;
    vcpu_fd.set_regs(&vcpu_regs)?;

    // 7. Run code on the vCPU.
    loop {
        // VMLAUNCH
        match vcpu_fd.run()? {
            // VM-EXIT
            VcpuExit::IoOut(addr, data) => {
                println!(
                    "Received an I/O OUT exit @ {:#x} - Data: [ASCII {:#x} -> '{}'])",
                    addr, data[0], data[0] as char
                );
            }
            VcpuExit::Hlt => {
                println!("Virtual CPU is HLTed");
                break;
            }
            r => panic!("Unexpected exit reason: {:?}", r),
        }
        // VMRESUME
    }

    Ok(())
}
