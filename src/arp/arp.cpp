#include "arp.h"

CppMacAddress cpp_send_arp(uint32_t remote_ip) {
  CppMacAddress mac_addr = {0};

  ULONG mac_buffer[2] = {0};
  ULONG mac_size = sizeof(mac_buffer);
  DWORD err_code = SendARP(remote_ip, 0, mac_buffer, &mac_size);

  if (NO_ERROR != err_code) {
    return mac_addr;
  }

  if (6 != mac_size) {
    return mac_addr;
  }

  memcpy(mac_addr.addr, mac_buffer, 6);
  return mac_addr;
}
