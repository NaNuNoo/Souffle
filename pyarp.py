import ipaddress
import netifaces
import scapy.all as scanet

onlineArray = []
def filterFunc(s, r):
	arpIp = r.sprintf('%ARP.psrc%')
	onlineArray.append(arpIp)
	return arpIp

for adapter in netifaces.interfaces():
	address = netifaces.ifaddresses(adapter)
	ip = address[netifaces.AF_INET][0].get('addr')
	mask = address[netifaces.AF_INET][0].get('netmask')
	mac = address[netifaces.AF_LINK][0].get('broadcast')
	if mask and mac:
		ip = ipaddress.IPv4Address(ip)
		mask = ipaddress.IPv4Address(mask)
		base = ipaddress.IPv4Address(int(ip) & int(mask))
		netText = '%s/%s' % (base, mask)
		network = ipaddress.IPv4Network(netText)
		print(network)
		ether = scanet.Ether(dst=mac)
		arp = scanet.ARP(pdst=str(network))
		ans, unans = scanet.srp(ether/arp, timeout = 5)
		ans.filter(filterFunc)

print(onlineArray)
