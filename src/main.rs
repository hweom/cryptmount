use std::env;
use std::process::{exit, Command};

fn print_usage(binary_name: &str) {
    println!("Usage:");
    println!(" {} open <device> <mount_point>", binary_name);
    println!(" {} close <device> <mount_point>", binary_name);
}

fn open(volume: &str, mountpoint: &str, mapper_device: &str) {
    println!("Running '/usr/bin/cryptsetup open {} {}", volume, mapper_device);
    let cryptsetup_output = Command::new("/usr/bin/cryptsetup")
        .arg("open")
        .arg(volume)
        .arg(mapper_device.clone())
        .spawn() // So that cryptsetup will get the password through STDIN of the parent process.
        .expect("Failed to spawn cryptsetup")
        .wait_with_output()
        .expect("Failed to run cryptsetup");

    println!("{:?}", cryptsetup_output);
    if !cryptsetup_output.status.success() {
        println!("Cryptsetup failed");
        exit(-1);
    }

    println!("Running '/usr/bin/fsck -y /dev/mapper/{}", mapper_device);
    let fsck_output = Command::new("/usr/bin/fsck")
        .arg("-y")
        .arg("/dev/mapper/".to_string() + &mapper_device)
        .output()
        .expect("Failed to fsck the partition");

    println!("{:?}", fsck_output);
    match fsck_output.status.code() {
        Some(0) => (), // No errors.
        Some(1) => (), // All errors corrected.
        _ => {
            println!("fsck failed");
            exit(-1);
        }
    }

    println!("Running '/usr/bin/mount /dev/mapper/{} {}", mapper_device, mountpoint);
    let mount_output = Command::new("/usr/bin/mount")
        .arg("/dev/mapper/".to_string() + &mapper_device)
        .arg(mountpoint)
        .output()
        .expect("Failed to mount the mapper device");

    println!("{:?}", mount_output);
    if !mount_output.status.success() {
        println!("Mount failed");
        exit(-1);
    }
}

fn close(mountpoint: &str, mapper_device: &str) {
    println!("Running '/usr/bin/umount {}", mountpoint);
    let mount_output = Command::new("/usr/bin/umount")
        .arg(mountpoint)
        .output()
        .expect("Failed to unmount the mapper device");

    println!("{:?}", mount_output);
    if !mount_output.status.success() {
        println!("Unmount failed");
        exit(-1);
    }

    println!("Running '/usr/bin/cryptsetup close {}", mapper_device);
    let cryptsetup_output = Command::new("/usr/bin/cryptsetup")
        .arg("close")
        .arg(mapper_device.clone())
        .output()
        .expect("Failed to run cryptsetup");

    println!("{:?}", cryptsetup_output);
    if !cryptsetup_output.status.success() {
        println!("Cryptsetup failed");
        exit(-1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_usage(&args[0]);
        exit(-1);
    }

    let operation = args.get(1).unwrap();
    let volume = args.get(2).unwrap();
    let mountpoint = args.get(3).unwrap();

    let mapper_device = &volume.replace("/", "_");

    match operation.as_ref() {
        "open" => open(&volume, &mountpoint, &mapper_device),
        "close" => close(&mountpoint, &mapper_device),
        _ => {
            print_usage(&args[0]);
            exit(-1)
        }
    }

    println!("Success");
}
