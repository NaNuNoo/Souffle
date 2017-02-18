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

void ip_num_to_str(uint32_t num, char* str) {
  uint8_t* buf = (uint8_t*)&num;
  sprintf(str, "%d.%d.%d.%d",
    buf[3], buf[2], buf[1], buf[0]);
}

void ip_str_to_num(const char* str, uint32_t* num) {
  uint32_t tmp[4];
  sscanf(str, "%d.%d.%d.%d",
    &tmp[3], &tmp[2], &tmp[1], &tmp[0]);
  uint8_t* buf = (uint8_t*)num;
  buf[0] = (uint8_t)tmp[0];
  buf[1] = (uint8_t)tmp[1];
  buf[2] = (uint8_t)tmp[2];
  buf[3] = (uint8_t)tmp[3];
}

struct AdapterInfo {
  uint32_t ip_address;
  uint32_t ip_mask;
  uint32_t gateway;
  uint8_t mac_address[6];
};

struct AdapterInfoGroup {
  uint32_t count;
  AdapterInfo* pointer;
};

AdapterInfoGroup* query_adapter_info() {
  DWORD err_code = 0;
  IP_ADAPTER_INFO* list_head = NULL;
  AdapterInfoGroup* group = NULL;

  ULONG list_size = 0;
  err_code = GetAdaptersInfo(NULL, &list_size);
  if (ERROR_BUFFER_OVERFLOW != err_code) {
    goto LABLE_ERROR;
  }

  list_head = (IP_ADAPTER_INFO*)malloc(list_size);
  err_code = GetAdaptersInfo(list_head, &list_size);
  if (ERROR_SUCCESS != err_code) {
    goto LABLE_ERROR;
  }

  uint32_t adp_count = 0;
  for (IP_ADAPTER_INFO* list_iter = list_head; NULL != list_iter; list_iter = list_iter->Next) {
    adp_count = adp_count + 1;
  }
  uint32_t group_size = sizeof(AdapterInfoGroup) + sizeof(AdapterInfo) * adp_count;
  group = (AdapterInfoGroup*)malloc(group_size);
  if (NULL == group) {
    goto LABLE_ERROR;
  }
  memset(group, 0, group_size);
  group->count = adp_count;
  group->pointer = (AdapterInfo*)(group + 1);

  adp_count = 0;
  for (IP_ADAPTER_INFO* list_iter = list_head; NULL != list_iter; list_iter = list_iter->Next) {
    AdapterInfo* adp_info = group->pointer + adp_count;
    adp_count = adp_count + 1;

    if (list_iter->AddressLength == 6) {
      for (UINT idx = 0; idx < list_iter->AddressLength; idx++) {
        adp_info->mac_address[idx] = list_iter->Address[idx];
      }
    }

    ip_str_to_num(list_iter->IpAddressList.IpAddress.String, &adp_info->ip_address);
    ip_str_to_num(list_iter->IpAddressList.IpMask.String, &adp_info->ip_mask);
    ip_str_to_num(list_iter->GatewayList.IpAddress.String, &adp_info->gateway);
  }

  free(list_head);
  return group;

LABLE_ERROR:
  if (NULL != list_head) {
    free(list_head);
  }
  if (NULL != group) {
    free(group);
  }
  return NULL;
}

void free_adapter_info(AdapterInfoGroup* group) {
  if (NULL != group) {
    free(group);
  }
}

struct ArpInfo {
  uint32_t ip_address;
  uint8_t mac_address[6];
};

ArpInfo send_arp(uint32_t remote_ip) {
  ArpInfo arp_info = {0};
  arp_info.ip_address = remote_ip;

  ULONG mac_buffer[2] = {0};
  ULONG mac_size = sizeof(mac_buffer);
  DWORD err_code = SendARP(arp_info.ip_address, NULL, mac_buffer, &mac_size);

  if (NO_ERROR != err_code) {
    return arp_info;
  }

  if (6 != mac_size) {
    return arp_info;
  }

  memcpy(arp_info.mac_address, mac_buffer, 6);
  return arp_info;
}

int main() {
  //query_adapter_info();
  int ip = inet_addr("192.168.1.7");
  send_arp(ip);
  return 0;
}
