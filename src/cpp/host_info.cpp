#include "host_info.h"

AdapterInfoGroup* query_adapter_info() {
  DWORD err_code = 0;
  IP_ADAPTER_INFO* list_head = nullptr;
  AdapterInfoGroup* group = nullptr;

  ULONG list_size = 0;
  err_code = GetAdaptersInfo(nullptr, &list_size);
  if (ERROR_BUFFER_OVERFLOW != err_code) {
    goto LABLE_ERROR;
  }

  list_head = (IP_ADAPTER_INFO*)malloc(list_size);
  err_code = GetAdaptersInfo(list_head, &list_size);
  if (ERROR_SUCCESS != err_code) {
    goto LABLE_ERROR;
  }

  uint32_t adp_count = 0;
  for (IP_ADAPTER_INFO* list_iter = list_head; nullptr != list_iter; list_iter = list_iter->Next) {
    adp_count = adp_count + 1;
  }
  uint32_t group_size = sizeof(AdapterInfoGroup) + sizeof(AdapterInfo) * adp_count;
  group = (AdapterInfoGroup*)malloc(group_size);
  if (nullptr == group) {
    goto LABLE_ERROR;
  }
  memset(group, 0, group_size);
  group->count = adp_count;
  group->pointer = (AdapterInfo*)(group + 1);

  adp_count = 0;
  for (IP_ADAPTER_INFO* list_iter = list_head; nullptr != list_iter; list_iter = list_iter->Next) {
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
  if (nullptr != list_head) {
    free(list_head);
  }
  if (nullptr != group) {
    free(group);
  }
  return nullptr;
}

void free_adapter_info(AdapterInfoGroup* group) {
  if (nullptr != group) {
    free(group);
  }
}
