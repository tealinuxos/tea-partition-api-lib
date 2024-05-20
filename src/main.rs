#![allow(unused)]
use std::{io::BufReader, process::{Command, Stdio}};
use std::io::prelude::*;

#[derive(Debug)]
struct Disk {
    path: String,
    size: String,
    model: String,
    transport: String,
    logical_sector_size: usize,
    physical_sector_size: usize,
    label: String,
    partitions: Vec<Partition>,
}

#[derive(Debug)]
struct Partition {
    number: usize,
    start: String,
    end: String,
    size: String,
    name: String,
    filesystem: String,
    flags: Vec<String>,
    // type
    // type_id
    // uuid
    // type_uuid
}

fn main() -> std::io::Result<()> {
    let mut app = Command::new("parted")
        .arg("-lm")
        .stdout(Stdio::piped())
        .spawn()?;

    let out = app.stdout.take().expect("unreachable");
    let mut disks = vec![];
    let mut reader = BufReader::new(out);
    let mut line = String::new();

    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 { break }

        // BYT;\n
        if n == 5 {
            line.clear();
            let _head = reader.read_line(&mut line)?;

            // there is one last columns, but all is empty, so idk whats there
            // i think its Disk Flags
            let mut cols = line.split(":").map(ToString::to_string);

            // logical and physical maybe flipped
            let disk = Disk {
                path: cols.next().unwrap_or_default(),
                size: cols.next().unwrap_or_default(),
                transport: cols.next().unwrap_or_default(),
                logical_sector_size: cols.next().unwrap_or_default().parse::<usize>().unwrap_or_default(),
                physical_sector_size: cols.next().unwrap_or_default().parse::<usize>().unwrap_or_default(),
                label: cols.next().unwrap_or_default(),
                model: cols.next().unwrap_or_default(),
                partitions: {
                    let mut partitions = vec![];

                    loop {
                        line.clear();
                        let n = reader.read_line(&mut line)?;
                        if n == 1 { break }

                        let mut cols = line.split(":").map(ToString::to_string);

                        partitions.push(Partition {
                            number: cols.next().unwrap_or_default().parse::<usize>().unwrap_or_default(),
                            start: cols.next().unwrap_or_default(),
                            end: cols.next().unwrap_or_default(),
                            size: cols.next().unwrap_or_default(),
                            filesystem: cols.next().unwrap_or_default(),
                            name: cols.next().unwrap_or_default(),
                            flags: {
                                let flags = cols.next().unwrap_or_default();
                                flags[..flags.len() - 2]
                                    .split(", ")
                                    .filter_map(|e|if e.is_empty() { None } else { Some(e.to_string()) })
                                    .collect::<Vec<String>>()
                            },
                        });
                    }

                    partitions
                }
            };

            disks.push(disk);
        }
    }

    println!("{disks:#?}");

    Ok(())
}

