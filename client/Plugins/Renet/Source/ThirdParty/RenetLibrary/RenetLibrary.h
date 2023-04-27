#ifndef RENET_LIB
#define RENET_LIB

#ifndef __cplusplus
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#endif

#ifdef __cplusplus
#include <cstdint> 
extern "C" {
#endif

bool rnet_get_result_status();
const char* rnet_get_result_message();
const char* rnet_get_result_detail();


void rnet_init_client();
uint64_t rnet_time_now();

const uint8_t** rnet_get_messages(uint64_t duration, uint8_t channel);

void rnet_send_messages();
void rnet_send_message(uint8_t channel_id, const char *msg, size_t msg_len);

uint8_t rnet_get_reliable_channel_id();
uint8_t rnet_get_unreliable_channel_id();
uint8_t rnet_get_chunk_channel_id();

#ifdef __cplusplus
}
#endif

#endif /* RENET_LIB */