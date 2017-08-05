#define _CRT_SECURE_NO_WARNINGS 1

#include <winsock2.h>
#include <iphlpapi.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#pragma comment(lib, "IPHLPAPI.lib")
#pragma comment(lib, "ws2_32.lib")

const int MAX_IP_STR = 16;
const int MAX_MAC_STR = 18;

VOID ipNumToStr(UINT num, CHAR* str) {
  BYTE* buf = (BYTE*)&num;
  sprintf(str, "%d.%d.%d.%d",
    buf[3], buf[2], buf[1], buf[0]);
}

VOID ipStrToNum(const CHAR* str, UINT* num) {
  UINT tmp[4];
  sscanf(str, "%d.%d.%d.%d",
    &tmp[3], &tmp[2], &tmp[1], &tmp[0]);
  BYTE* buf = (BYTE*)num;
  buf[0] = (BYTE)tmp[0];
  buf[1] = (BYTE)tmp[1];
  buf[2] = (BYTE)tmp[2];
  buf[3] = (BYTE)tmp[3];
}

VOID macBufToStr(const BYTE* buf, CHAR* str) {
  sprintf(str, "%02X-%02X-%02X-%02X-%02X-%02X",
    buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]);
}

struct LocalAdapter {
  uint8_t ip[4];
  uint8_t mask[4];
  uint8_t mac[6];
};

const LocalAdapter* ScanLocalAdapter() {
  DWORD result = 0;

  ULONG adapterSize = 0;
  result = GetAdaptersInfo(NULL, &adapterSize);
  if (ERROR_BUFFER_OVERFLOW != result) {
    return 0;
  }

  IP_ADAPTER_INFO* adapterHead = NULL;
  adapterHead = (IP_ADAPTER_INFO*)malloc(adapterSize);
  result = GetAdaptersInfo(adapterHead, &adapterSize);
  if (ERROR_SUCCESS != result) {
    return 0;
  }

  for (IP_ADAPTER_INFO* adapterItr = adapterHead; NULL != adapterItr; adapterItr = adapterItr->Next) {
    printf("\tAdapter Name: \t%s\n", adapterItr->AdapterName);
    printf("\tAdapter Desc: \t%s\n", adapterItr->Description);
    printf("\tAdapter Addr: \t");
    for (UINT idx = 0; idx < adapterItr->AddressLength; idx++) {
      if (idx == (adapterItr->AddressLength - 1)) {
        printf("%.2X\n", (UINT)adapterItr->Address[idx]);
      } else {
        printf("%.2X-", (UINT)adapterItr->Address[idx]);
      }
    }

    printf("\tIP Address: \t%s\n", adapterItr->IpAddressList.IpAddress.String);
    printf("\tIP Mask: \t%s\n", adapterItr->IpAddressList.IpMask.String);
    printf("\tGateway: \t%s\n", adapterItr->GatewayList.IpAddress.String);

    printf("\n");
  }
}

struct LocalNetWork {

};

void GetNetworkInfo(UINT subnet, UINT mask) {
  UINT maskSize = 1;
  for (UINT idx = 0; idx < 32; ++idx) {
    if (mask & (0x1 << idx)) {
      break;
    }
    maskSize = maskSize * 2;
  }

  for (UINT idx = 0; idx < maskSize - 1; ++idx) {
    CHAR ipStr[MAX_IP_STR];
    ipNumToStr(subnet | idx, ipStr);
    IPAddr ipAddr = inet_addr(ipStr);

    ULONG macBuf[2];
    ULONG macSize = sizeof(macBuf);
    DWORD result = SendARP(ipAddr, NULL, macBuf, &macSize);
    if (NO_ERROR == result) {
      CHAR macStr[MAX_MAC_STR];
      macBufToStr((BYTE*)macBuf, macStr);
      printf("%s", macStr);
    }
  }
}

int main() {
  UINT ip = 0;
  ipStrToNum("192.168.1.0", &ip);

  UINT mask = 0;
  ipStrToNum("255.255.255.192", &mask);

  GetNetworkInfo(ip, mask);
  return 0;
}