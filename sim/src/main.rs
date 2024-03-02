extern crate getopt;
use getopt::Parser;
use std::{env, fs::File, io::{BufRead, BufReader}};
// use std::collections::HashMap;

#[derive(Clone)]
struct CacheLine {
    valid: bool,
    tag: usize,
    last_used: usize,
}

struct Cache {
    lines: Vec<CacheLine>,
    e: usize,
}

impl Cache {
    fn new(set_size: usize, lines_per_set: usize) -> Self {
        Cache {
            lines: vec![CacheLine { valid: false, tag: 0, last_used: 0 }; set_size * lines_per_set],
            e: lines_per_set,
        }
    }

    fn access(&mut self, set_index: usize, tag: usize, time: &mut usize) -> (bool, bool) { // (hit, eviction)
        let start = set_index * self.e;
        let end = start + self.e;
        let mut min_time_index = start;
        let mut found_invalid = false;

        for i in start..end {
            if self.lines[i].valid && self.lines[i].tag == tag {
                self.lines[i].last_used = *time;
                *time += 1;
                return (true, false);
            }
            if !self.lines[i].valid {
                found_invalid = true;
            }
            if self.lines[min_time_index].last_used > self.lines[i].last_used {
                min_time_index = i;
            }
        }

        if !found_invalid {
            self.lines[min_time_index].valid = true;
            self.lines[min_time_index].tag = tag;
            self.lines[min_time_index].last_used = *time;
            *time += 1;
            return (false, true);
        }

        for i in start..end {
            if !self.lines[i].valid {
                self.lines[i].valid = true;
                self.lines[i].tag = tag;
                self.lines[i].last_used = *time;
                *time += 1;
                return (false, false);
            }
        }

        unreachable!();
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Parser::new(&args, "hvs:E:b:t:");
    let mut s = 0;
    let mut e = 0;
    let mut b = 0;
    let mut trace_file = String::new();
    let mut verbose = false;

    while let Some(result) = opts.next() {
        match result {
            Ok(opt) => match opt {
                getopt::Opt('h', _) => {
                    println!("Usage: ./sim [-hv] -s <s> -E <E> -b <b> -t <tracefile>");
                    return;
                },
                getopt::Opt('v', _) => {
                    verbose = true;
                },
                getopt::Opt('s', Some(arg)) => {
                    s = arg.parse().unwrap();
                },
                getopt::Opt('E', Some(arg)) => {
                    e = arg.parse().unwrap();
                },
                getopt::Opt('b', Some(arg)) => {
                    b = arg.parse().unwrap();
                },
                getopt::Opt('t', Some(arg)) => {
                    trace_file = arg;
                },
                _ => {}
            },
            Err(_) => {
                // ここでエラーを処理する
                println!("Invalid option encountered.");
                return;
            }
        }
    }


    let num_sets = 1 << s;
    let mut cache = Cache::new(num_sets, e);
    let mut time = 0;
    let mut hits = 0;
    let mut misses = 0;
    let mut evictions = 0;

    let file = File::open(trace_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('I') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let operation = parts[0].chars().next().unwrap();
        let address_str = parts[1].split(',').next().unwrap();
        let address = usize::from_str_radix(&address_str, 16).unwrap();
        let set_index = (address >> b) & ((1 << s) - 1);
        let tag = address >> (s + b);
        let size = parts[1].split(',').last().unwrap();

        match operation {
            'L' | 'S' => {
                let (hit, eviction) = cache.access(set_index, tag, &mut time);
                if hit {
                    hits += 1;
                    if verbose {
                        println!("{} {},{} hit", operation, address_str, size);
                    }
                } else {
                    misses += 1;
                    if eviction {
                        evictions += 1;
                    }
                    if verbose {
                        println!("{} {},{} miss{}", operation, address_str, size, if eviction { " eviction" } else { "" });
                    }
                }
            }
            'M' => {
                let (hit, eviction) = cache.access(set_index, tag, &mut time);
                if hit {
                    // Modify操作がヒットした場合、ロードとストアの両方でヒットするため、hitsを2回増やす
                    hits += 2;
                    if verbose {
                        println!("{} {},{} hit hit", operation, address_str, size);
                    }
                } else {
                    // Modify操作がミスした場合、最初はミス、その後の書き込みでヒット
                    misses += 1;
                    if eviction {
                        evictions += 1;
                    }
                    hits += 1; // 書き込みでヒット
                    if verbose {
                        println!("{} {},{} miss{} hit", operation, address_str, size, if eviction { " eviction" } else { "" });
                    }
                }
            }
            _ => {}
        }
    }

    println!("hits:{} misses:{} evictions:{}", hits, misses, evictions);
}
