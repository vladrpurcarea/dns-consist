use crate::opts::Analyze;
use crate::lists::*;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use std::str::FromStr;
use std::fs;
use std::collections::HashMap;
use std::thread;
use std::error::Error;
use std::cmp;

#[derive(Debug)]
pub struct TestResult {
    pub server: DNSServer,
    pub result: Result<Vec<Ipv4Addr>, Box<dyn Error + Send + Sync>>,
}

const MAX_THREADS: usize = 500;

pub fn analyze(cmd: Analyze) {
    let mut server_list: Vec<DNSServer> = prepare_server_list(cmd.full);
    cmd.max_servers.map(|max| server_list.truncate(max));

    let num_threads = cmd.threads.unwrap_or(cmp::min(MAX_THREADS, server_list.len()));
    let chunk_size = server_list.len() / num_threads;
    let mut threads = vec![];
    for chunk in server_list.chunks(chunk_size) {
	let cmd = cmd.clone();
	let chunk = chunk.to_vec();
	threads.push(thread::spawn(move || -> Vec<TestResult> {
	    chunk.iter().map(|server| do_test(server, &cmd.website)).collect()
	}));
    }

    let mut results: Vec<TestResult> = vec![];
    for thread in threads {
	let mut thread_results: Vec<TestResult> = thread.join().expect("Thread failed");
	results.append(&mut thread_results);
    }
    analyze_results(&results);
}

struct IpStats {
    count: u64,
    countries: Vec<String>,
    server_names: Vec<String>
}
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
    let mut ip_stats = HashMap::<Ipv4Addr, IpStats>::new();
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
	}
    }
    
    for (ip, stats) in ip_stats.iter() {
	println!("{: <16} {:?} {:?} {:?}", ip, stats.count, stats.countries, &stats.server_names[0..(cmp::min(5, stats.server_names.len()))]);
    }
}

fn prepare_server_list(full: bool) -> Vec<DNSServer> {
    // read csv from ~/.dns-consist/dnslist.csv
    let csv_file = fs::read_to_string(get_server_list_path())
	.expect("Could not read server list. Did you run dns-consist update-dns-lists?");
    let servers = parse_dns_csv(&csv_file)
	.expect("Failed to parse CSV file. Try updating it: dns-consist update-dns-lists");

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
    
