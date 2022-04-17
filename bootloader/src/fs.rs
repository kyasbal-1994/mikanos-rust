use alloc::vec::Vec;
use alloc::boxed::Box;

use core::borrow::{Borrow, BorrowMut};
use log::info;
use uefi::{CStr16, Handle};
use uefi::prelude::BootServices;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileInfo, FileMode, FileType, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn open_root_dir(image: Handle, bs: &BootServices) -> Directory {
    get_simple_file_system(image, bs).open_volume().unwrap()
}

fn open(dir: &mut Directory, filename: &str) -> FileType{
    let mut cstr_buf = [0u16; 32];
    let cstr_file_name = CStr16::from_str_with_buf(filename,&mut cstr_buf).unwrap();
    dir.open(cstr_file_name,FileMode::Read,FileAttribute::empty())
        .unwrap()
        .into_type()
        .unwrap()
    
}

fn get_file_info(file: &mut impl File) ->Box<FileInfo> {
    file.get_boxed_info::<FileInfo>().unwrap()
}

pub fn open_file(dir: &mut Directory, filename: &str) -> RegularFile {
    match open(dir,filename) {
        FileType::Regular(file)=>file,
        FileType::Dir(_) => panic!("Not a regular file: {}", filename)
    }
}

pub fn read_file_to_vec(file: &mut RegularFile) -> Vec<u8> {
    let size = get_file_info(file).file_size() as usize;
    let mut buf = vec![0; size];
    file.read(&mut buf).unwrap();
    buf
}

fn get_simple_file_system(image: Handle, bs: &BootServices) -> &mut SimpleFileSystem{
    let sfs = bs.get_image_file_system(image).unwrap();
    unsafe { sfs.interface.get().as_mut() }.unwrap()
}