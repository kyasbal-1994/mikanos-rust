//
// Boot loader for Mikan OS written in Rust
// This file is strongly influenced by https://github.com/yubrot/ors
//
#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
mod fs;

#[macro_use]
extern crate alloc;

extern crate uefi_services;

use core::fmt::Write;
use core::{mem, slice};
use core::arch::asm;
use core::borrow::Borrow;
use goblin::elf;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use log::info;
use shared::framebuffer;
use uefi::proto::console::gop::{GraphicsOutput,PixelFormat};

const EFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main(_image: Handle,mut st: SystemTable<Boot>)-> Status {
    uefi_services::init(&mut st).unwrap();
    st.stdout().reset(false).unwrap();
    dump_memory_map(&st);

    let kernel_entry = load_kernel("kernel.elf",_image,&st);
    info!("kernel entry: {}",kernel_entry);
    let fb = get_frame_buffer(st.boot_services());
    let entry_point: extern "sysv64" fn(&framebuffer::FrameBuffer) = unsafe {
        mem::transmute(kernel_entry)
    };

    entry_point(&fb);
    info!("After kernel call");
    loop{
        unsafe {
            asm!("hlt");
        }
    }
}

fn load_kernel(path: &str, image: Handle, st: &SystemTable<Boot>)->usize {
    let mut root_dir = fs::open_root_dir(image,st.boot_services());
    let mut file = fs::open_file(&mut root_dir, path);
    let buf = fs::read_file_to_vec(&mut file);
    load_elf(&buf,st)
}

fn load_elf(src: &[u8], st: &SystemTable<Boot>)->usize{
    let elf = elf::Elf::parse(src).unwrap();

    let mut dest_start = usize::MAX;
    let mut dest_end = 0;
    for ph in elf.program_headers.iter() {

        info!("Program header: {} {} {} {}",elf::program_header::pt_to_str(ph.p_type),ph.p_offset,ph.p_vaddr,ph.p_memsz);

        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        dest_start = dest_start.min(ph.p_vaddr as usize);
        dest_end = dest_end.max(ph.p_vaddr + ph.p_memsz);
    }

    st.boot_services().allocate_pages(AllocateType::Address(dest_start),MemoryType::LOADER_DATA,(dest_end as usize - dest_start as usize + EFI_PAGE_SIZE - 1) / EFI_PAGE_SIZE);

    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }

        let ofs = ph.p_offset as usize;
        let fsize = ph.p_filesz as usize;
        let msize = ph.p_memsz as usize;

        let dest = unsafe {
            slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, msize)
        };

        dest[..fsize].copy_from_slice(&src[ofs..ofs+fsize]);
        dest[fsize..].fill(0);
    }
    elf.entry as usize
}

fn get_frame_buffer(bs: &BootServices)->framebuffer::FrameBuffer {
    let mut gopCell = bs.locate_protocol::<GraphicsOutput>().unwrap();
    let mut gop = unsafe{ &mut *gopCell.get() };
    let mut  ptr = gop.frame_buffer().as_mut_ptr();
    framebuffer::FrameBuffer{
        framebuffer: gop.frame_buffer().as_mut_ptr(),
        stride: gop.current_mode_info().stride() as u32,
        resolution: (
            gop.current_mode_info().resolution().0 as u32,
            gop.current_mode_info().resolution().1 as u32
            ),
        format: match gop.current_mode_info().pixel_format() {
            PixelFormat::Rgb=>framebuffer::PixelFormat::Rgb,
            PixelFormat::Bgr=>framebuffer::PixelFormat::Bgr,
            f=>panic!("Unsupported pixel format :{:?}",f)
        }
    }
}

fn dump_memory_map(st: &SystemTable<Boot>) {
    let enough_mmap_size = st.boot_services().memory_map_size().map_size + 8 * mem::size_of::<MemoryDescriptor>();
    let mut mmap_buf = vec![0; enough_mmap_size];
    let (_,descriptors) = st.boot_services().memory_map(&mut mmap_buf).unwrap();

    for (i,d) in descriptors.enumerate() {
        info!("{}, {:x}, {:?}, {:08x}, {:x}, {:x}",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff)
    }
}