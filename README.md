# dns-consist

`dns-consist` queries a list of DNS servers (retrieved from `public-dns.info`) and reports differences in the responses of different countries. This is useful for analyzing censored websites.

# Usage

	dns-consist update-dns-list # for most accurate results do this before every analyze
	
	dns-consist analyze --website example.com
	dns-consist analyze --website example.com --full # entire DNS list
	
	dns-consist analyze -w example.com --threads 50 --max-servers 100
	
A `--full` scan queries about 5.5k servers in around a minute on my hardware and connection.

You can control the number of threads with `--threads`. Since the workload is basically entirely IO-bound, upping this number usually speeds up the test.
When no argument is supplied, the test has a limit of 500 threads (but may use less if the number of servers is lower).



