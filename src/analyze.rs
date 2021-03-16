use crate::opts::Analyze;
use crate::lists::*;
use crate::exit;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use std::str::FromStr;
use std::fs;
use std::fs::File;
use std::collections::HashMap;
use std::thread;
use std::error::Error;
use std::cmp;
use std::cmp::Ordering;
use std::io::Write;

#[derive(Debug)]
pub struct TestResult {
    pub server: DNSServer,
    pub result: Result<Vec<Ipv4Addr>, Box<dyn Error + Send + Sync>>,
}

const MAX_THREADS: usize = 500;

pub fn analyze(cmd: Analyze) {
    // prepare work
    let mut server_list: Vec<DNSServer> = prepare_server_list(cmd.full);
    cmd.max_servers.map(|max| server_list.truncate(max));

    // spawn threads and do all dns requests
    let num_threads = cmd.threads.unwrap_or(cmp::min(MAX_THREADS, server_list.len()));
    if num_threads > server_list.len() {
	exit!("Number of threads must be smaller than number of DNS servers to be tested.");
    }
    let chunk_size = server_list.len() / num_threads;
    let mut threads = vec![];
    for chunk in server_list.chunks(chunk_size) {
	let cmd = cmd.clone();
	let chunk = chunk.to_vec();
	threads.push(thread::spawn(move || -> Vec<TestResult> {
	    chunk.iter().map(|server| do_test(server, &cmd.website)).collect()
	}));
    }

    // join threads and analyze results
    let mut results: Vec<TestResult> = vec![];
    for thread in threads {
	let mut thread_results: Vec<TestResult> = thread.join().expect("Thread failed.");
	results.append(&mut thread_results);
    }
    analyze_results(&results);

    // write out test results to output file if argument provided
    if let Some(file) = cmd.output_file {
	let mut f = File::create(file).unwrap_or_else(|_| exit!("Could not create output file"));
	for r in results {
	    f.write_all(format!("{:?}\n", r).as_bytes())
		.unwrap_or_else(|_| exit!("Failed writing output file"));
	}
    }
}

#[derive(Clone)]
struct IpStats {
    count: u64,
    countries: Vec<String>,
    server_names: Vec<String>}

impl IpStats {
    fn new() -> IpStats {
	IpStats {
	    count: 0,
	    countries: Vec::new(),
	    server_names: Vec::new()
	}
    }
}

fn analyze_results(results: &[TestResult]) {
    // go through result list and sum up ip counts, which countries/DNSes produced them
    let mut ip_stats = HashMap::<Ipv4Addr, IpStats>::new();
    let mut errors = 0;
    for r in results {
	let country = &r.server.country_code;
	if r.result.is_ok() {
	    r.result.as_ref().unwrap().iter().for_each(|ip| {
		ip_stats.entry(*ip).or_insert(IpStats::new());
		let entry = ip_stats.get_mut(ip).unwrap();
		entry.count += 1;
		if !entry.countries.contains(&country) {
		    entry.countries.push(country.clone())
		}
		r.server.name.as_ref().map(|name| entry.server_names.push(name.to_string()));
	    });
	} else {
	    errors += 1;
	}
    }

    // order ip stats
    let mut ord_ips = Vec::<(Ipv4Addr, IpStats)>::new();
    for (ip, stats) in ip_stats.iter() {
	ord_ips.push((*ip, stats.clone()));
    }
    ord_ips.sort_by(|a, b| {
	let res = b.1.count.cmp(&a.1.count);
	match res {
	    Ordering::Equal => a.0.cmp(&b.0),
	    _ => res
	}
    });

    // print
    for (ip, stats) in ord_ips {
	println!("{: <16} {:?} {:?} {:?}", ip, stats.count, stats.countries, &stats.server_names[0..(cmp::min(5, stats.server_names.len()))]);
    }
    println!("Failed requests: {}", errors);
}

fn prepare_server_list(full: bool) -> Vec<DNSServer> {
    // read csv from ~/.dns-consist/dnslist.csv
    let csv_file = fs::read_to_string(get_server_list_path())
	.unwrap_or_else(|_| exit!("Could not read server list. Did you run dns-consist update-dns-lists?"));
    let servers = parse_dns_csv(&csv_file)
	.unwrap_or_else(|_| exit!("Failed to parse CSV file. Try updating it: dns-consist update-dns-lists"));

    match full {
	true => servers, // return all servers
	false => {
	    // return only 3 servers per country
	    let mut res = Vec::<DNSServer>::new();
	    let mut count = HashMap::<String, u32>::new();
	    for s in servers {
		match count.get_mut(&s.country_code) {
		    Some(c) => *c += 1,
		    None => { count.insert(s.country_code.clone(), 0); () }
		}
		if *(count.get(&s.country_code).unwrap()) <= 3 {
		    res.push(s);
		}
	    }
	    res
	}
    }
}

fn do_test(server: &DNSServer, website: &str) -> TestResult {
    TestResult {
	result: match server.ip_address.parse::<IpAddr>() {
	    Ok(ip) => dns_request(SocketAddr::new(ip, 53), &website),
	    Err(e) => Err(Box::new(e))
	},
	server: server.clone()
    }
}

fn dns_request(server_address: SocketAddr, query: &str) -> Result<Vec<Ipv4Addr>, Box<dyn Error + Send + Sync>> {
    // setup connection
    let conn = UdpClientConnection::new(server_address)?;
    let client = SyncClient::new(conn);

    // do query
    let name = Name::from_str(query)?;
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A)?;
    let answers: &[Record] = response.answers();

    // get ips
    let mut ips = Vec::<Ipv4Addr>::new();
    for a in answers {
	if let &RData::A(ref ip) = a.rdata() {
	    ips.push(*ip);
	}
    }
    
    Ok(ips)
}
    
