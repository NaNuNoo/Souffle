#include "util.h"

extern "C" struct AdapterInfo {
  uint32_t ip_address;
  uint32_t ip_mask;
  uint32_t gateway;
  uint8_t mac_address[6];
};

extern "C" struct AdapterInfoGroup {
  uint32_t count;
  AdapterInfo* pointer;
};

extern "C" AdapterInfoGroup* query_adapter_info();

extern "C" void free_adapter_info(AdapterInfoGroup* group);
