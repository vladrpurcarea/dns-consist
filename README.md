# dns-consist

`dns-consist` queries a list of DNS servers (retrieved from `public-dns.info`) and reports differences in the responses of different servers. This is useful for analyzing which websites are censored where, as a common technique for implementing blocks is to order ISPs to route all DNS responses to a dedicated block page.

# Usage

	dns-consist update-dns-list # for most accurate results do this before every analyze
	
	dns-consist analyze --website example.com
	dns-consist analyze --website example.com --full # entire DNS list
	
	dns-consist analyze -w example.com --threads 50 --max-servers 100
    
Output example:

	192.0.2.0        3 ["US"] ["dns.example.com."]
	192.0.3.0        2 ["FR"] ["dns2.example.com.", "dns3.example.com"]
	
The columns are:

* the ip address reported to be the queried domain's
* the number of times it appears (in general)
* the country of the DNS
* the hostname (if available) of the DNS
	
A `--full` scan queries about 5.5k servers in around a minute on my hardware and connection.

You can control the number of threads with `--threads`. When no argument is supplied, the test has a limit of 500 threads (but may use less if the number of servers is lower).

Upping the thread count to 5k reduces the runtime to ~6s on my machine. Bear in mind you will likely need to increase the file descriptor limit on your system to be able to do this 
(in `/etc/security/limits.conf` on the *nixes).

# Findings

Disclaimer: being on the list does not necessarily mean a domain is blocked country-wide, or even for a significant portion. Or at all. 

## Sci-Hub

	127.0.0.1        11 ["IT", "FR", "MQ", "US"] ["ns.unisal.it.", "83-103-61-107.ip.fastwebnet.it.", "85-18-246-73.ip.fastwebnet.it.", "194.243.154.62.", "mail.abeaform.it."]
	212.97.32.22     7 ["IT"] ["ns2.infracom.it.", "dns1.kpnqwest.it.", "dns2.kpnqwest.it.", "ns4.eutelia.it.", "primary.netscalibur.it."]
	213.224.83.39    6 ["BE"] ["d5152c562.static.telenet.be.", "d51530cfd.static.telenet.be.", "d51531217.static.telenet.be.", "d51531251.static.telenet.be.", "d51531381.static.telenet.be."]
	188.186.157.49   4 ["RU"] ["dynamicip-157-144-91-237.pppoe.chelny.ertelecom.ru.", "84-244-59-15.sibtele.com.", "213.10.221.176.telrostelecom.ru."]
	62.149.188.252   3 ["IT"] ["mail.conadfo.it.", "dns.aruba.it.", "dns4.aruba.it."]
	88.198.37.7      2 ["IR"] ["dns.shecan.ir."]
	176.9.122.179    2 ["IR"] ["dns.shecan.ir."]
	176.9.122.183    2 ["IR"] ["dns.shecan.ir."]
	195.23.113.202   2 ["PT"] ["78-130-39-4.static.net.nos.pt.", "62.169.90.238.rev.optimus.pt."]
	213.13.145.120   2 ["PT"] ["static-wan-bl3-227-166.rev.webside.pt.", "static-wan-bl1-250-106-rev.webside.pt."]
	213.33.66.164    2 ["AT"] ["res1.a1.net."]
	31.204.161.41    1 ["RU"] ["ab-31-204-180-44.mxc.ru."]
	85.142.29.248    1 ["RU"] ["mail-gw2.portal.amsd.com."]
	146.112.61.106   1 ["US"] []
	156.154.175.221  1 ["US"] []
	156.154.176.221  1 ["US"] []
	193.58.251.1     1 ["RU"] ["dns.skydns.ru."]
	195.191.182.2    1 ["RU"] []
	195.208.152.206  1 ["RU"] ["ip91.217.62.219.ipblk.stnsk.ru."]
	195.238.1.190    1 [""] []
	212.19.108.4     1 ["IT"] ["dns.leonet.it."]

The first row shows a common technique, redirecting to `127.0.0.1`. 

Amusingly, skydns (`RU`) redirects to.. an advertisement for country wide content filtering and surveillance.

## Library Genesis

	127.0.0.1        16 ["IT", "FR", "MQ", "US"] ["ns.unisal.it.", "83-103-61-107.ip.fastwebnet.it.", "85-18-246-73.ip.fastwebnet.it.", "194.243.154.62.", "mail.abeaform.it."]
	212.97.32.22     6 ["IT"] ["ns2.infracom.it.", "dns1.kpnqwest.it.", "dns2.kpnqwest.it.", "ns4.eutelia.it.", "primary.netscalibur.it."]
	213.224.83.39    5 ["BE"] ["d5152c562.static.telenet.be.", "d5152c76f.static.telenet.be.", "d51530cfd.static.telenet.be.", "d51531217.static.telenet.be.", "d51531251.static.telenet.be."]
	195.23.113.202   4 ["PT"] ["62.169.90.150.rev.optimus.pt.", "62.169.90.154.rev.optimus.pt.", "78-130-39-4.static.net.nos.pt.", "78-130-39-188.static.net.nos.pt."]
	62.149.188.252   2 ["IT"] ["dns.aruba.it.", "dns4.aruba.it."]
	146.112.61.106   1 ["US"] []
		
Sci-Hub's cousin. DNS blocked by the same countries largely.

## The Pirate Bay

	10.10.34.35      87 ["IR"] ["dns.rooyekhat.co.", "dns-fw2.baharnet.ir.", "ns3.iranet.ir.", "dns.shecan.ir.", "ns1.farzaneganpars.ir."]
	127.0.0.1        20 ["IT", "BE", "MQ", "FR", "US"] ["ns.unisal.it.", "83-103-61-107.ip.fastwebnet.it.", "cache0100.ns.eu.uu.net.", "ns1.nshq.nato.int.", "85-18-246-73.ip.fastwebnet.it."]
	213.224.83.39    7 ["BE"] ["d5152c562.static.telenet.be.", "d5152c76f.static.telenet.be.", "d51530cfd.static.telenet.be.", "d51531217.static.telenet.be.", "d51531251.static.telenet.be."]
	101.167.164.53   6 ["AU"] ["uneeda.telstra.net.", "lon-resolver.telstra.net."]
	101.167.166.53   6 ["AU"] ["uneeda.telstra.net.", "lon-resolver.telstra.net."]
	10.10.34.36      4 ["IR"] ["ns1.rasanapishtaz.ir.", "ns2.rasanapishtaz.ir."]
	202.162.209.133  4 ["ID"] ["226.17.100.121.iconpln.net.id.", "186.78.3.103.iconpln.net.id.", "ip-89.157.tmb.diklat.esdm.go.id."]
	0.0.0.0          3 ["HU", "FI", "DE"] ["124.104-38-194.hosting.adatpark.hu.", "dns.brahma.world.", "pi-1.it-sos-bonn.de."]
	165.21.74.33     3 ["SG"] []
	188.186.157.49   3 ["RU"] ["dynamicip-157-144-91-237.pppoe.chelny.ertelecom.ru.", "84-244-59-15.sibtele.com.", "213.10.221.176.telrostelecom.ru."]
	202.152.0.14     3 ["ID"] []
	31.13.76.99      2 ["CN"] []
	31.13.80.17      2 ["CN"] []
	62.149.188.252   2 ["IT"] ["dns.aruba.it.", "dns4.aruba.it."]
	66.220.155.12    2 ["CN"] []
	81.196.9.138     2 ["RO"] ["dns-cache-1.rdsnet.ro.", "dns-cache-2.rdsnet.ro."]
	82.215.19.201    2 ["NL"] ["unlabelled-58-128-213-87.versatel.net."]
	85.142.29.248    2 ["RU"] ["mail-gw2.portal.amsd.com.", "mail.atik.ru."]
	146.112.61.106   2 ["US"] ["elgin.student.ssc.edu."]
	156.154.112.16   2 ["US"] []
	156.154.113.16   2 ["US"] []
	156.154.175.30   2 ["US"] []
	156.154.176.30   2 ["US"] []
	195.23.113.202   2 ["PT"] ["62.169.90.150.rev.optimus.pt.", "78-130-39-4.static.net.nos.pt."]
	199.96.58.157    2 ["MM"] []
	213.13.145.120   2 ["PT"] ["static-wan-bl3-227-166.rev.webside.pt.", "static-wan-bl1-250-106-rev.webside.pt."]
	213.33.66.164    2 ["AT"] ["res1.a1.net."]
	27.54.116.70     1 ["ID"] ["ip-103-75-210-3.moratelindo.net.id."]
	31.13.64.49      1 ["CN"] []
	31.13.72.1       1 ["CN"] []
	31.13.73.23      1 ["CN"] []
	31.13.74.17      1 ["CN"] ["public-dns-a.baidu.com."]
	148.123.15.40    1 ["NO"] ["dctrh001.powelasa.powel.com."]
	175.139.142.25   1 ["MY"] []
	185.45.6.57      1 ["CN"] ["pub1.sdns.360.cn."]
	192.168.241.52   1 ["RU"] ["25-126.vpn.serpuhov.biz."]
	193.113.9.167    1 ["GB"] ["host81-149-21-254.in-addr.btopenworld.com."]
	202.51.102.120   1 ["ID"] []
	.. many chinese servers excluded ..

The Pirate Bay is the most blocked website in the world. 

Big shoutout to `202.162.209.133` (Indonesia), which makes you watch an ad before telling you the website you wanted to visit is blocked. It boggles the mind.

Another interesting block page is `193.113.9.167` (United Kingdom), which leads to http://www.ukispcourtorders.co.uk/, a list of UK censored websites.

The Netherlands version `82.215.19.201` is similar, but contains all of the blocked Pirate Bay look-alikes, mirrors and clones.

## (North) Korean Association of Cooks

	5.79.87.160      1 ["NL"] []
	10.10.34.34      1 ["IR"] ["dns.shecan.ir."]
	80.241.219.22    1 ["DE"] ["-."]
	109.123.78.72    1 ["GB"] []
	127.0.0.1        1 ["US"] ["pns101.cloudns.net."]
	210.102.126.185  1 ["KR"] []
	
Although I was aware of it being blocked in South Korea, I did not expect to find DNS blocks in so many European countries. This is a common theme among blocked North Korean websites

The Korean blocked IP (`210.102.126.185`) is.. just a plain page with "It works" written in h1? It confuses me.

## Air Koryo (North Korean airline)

	5.79.87.160      1 ["NL"] []
	80.241.219.22    1 ["DE"] ["-."]
	94.126.23.10     1 ["CH"] []
	109.123.78.72    1 ["GB"] []
	127.0.0.1        1 ["US"] ["pns101.cloudns.net."]
	210.102.126.185  1 ["KR"] []


## (North) Korea Elderly Care Fund

For brevity, this is the same as above. You can view a list of such blocked websites at: https://en.wikipedia.org/wiki/List_of_North_Korean_websites_banned_in_South_Korea

## Best Block Page

Sweden's Bahnhof: http://5.150.254.31/
