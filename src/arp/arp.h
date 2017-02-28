#define _CRT_SECURE_NO_WARNINGS 1

#include <winsock2.h>
#include <iphlpapi.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <memory.h>

#pragma comment(lib, "iphlpapi.lib")
#pragma comment(lib, "ws2_32.lib")

extern "C" {
  struct CppMacAddress {
    uint8_t addr[6];
  };

  CppMacAddress cpp_send_arp(uint32_t remote_ip);
}
