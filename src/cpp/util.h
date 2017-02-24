#define _CRT_SECURE_NO_WARNINGS 1

#include <winsock2.h>
#include <iphlpapi.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <memory.h>
#pragma comment(lib, "iphlpapi.lib")
#pragma comment(lib, "ws2_32.lib")

const uint32_t IP_STR_SIZE = 16;
const uint32_t MAC_STR_SIZE = 16;

inline void ip_num_to_str(uint32_t num, char* str) {
  uint8_t* buf = (uint8_t*)&num;
  sprintf(str, "%d.%d.%d.%d",
    buf[3], buf[2], buf[1], buf[0]);
}

inline void ip_str_to_num(const char* str, uint32_t* num) {
  uint32_t tmp[4];
  sscanf(str, "%d.%d.%d.%d",
    &tmp[3], &tmp[2], &tmp[1], &tmp[0]);
  uint8_t* buf = (uint8_t*)num;
  buf[0] = (uint8_t)tmp[0];
  buf[1] = (uint8_t)tmp[1];
  buf[2] = (uint8_t)tmp[2];
  buf[3] = (uint8_t)tmp[3];
}
