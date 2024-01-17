enum DiskType {
    SDD,
    HDD,
}
#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let disk_type = DiskType::SDD;
    // Can't compare them like this!
    // if disk_type == DiskType::SDD {
    //     println!("Disk type is SDD");
    // } else {
    //     println!("Disk type is HDD");
    // }
    match disk_type {
        DiskType::SDD => println!("Disk type is SDD."),
        DiskType::HDD => println!("Disk type is HDD."),
    }

    let disk_size = DiskSize::GB((128));
    println!("{:?}", disk_size);
}
