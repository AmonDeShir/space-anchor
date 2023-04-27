// Fill out your copyright notice in the Description page of Project Settings.


#include "RenetClient.h"
#include "RenetLibrary/RenetLibrary.h"

// Sets default values
ARenetClient::ARenetClient()
{
 	// Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;
}

// Called when the game starts or when spawned
void ARenetClient::BeginPlay()
{
	Super::BeginPlay();
	rnet_init_client();

	if (!rnet_get_result_status()) {
		UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
		return;
	}

	rnet_init_client();

	if (!rnet_get_result_status()) {
		UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
		return;
	}

	lastUpdate = rnet_time_now();
	lastCommunication = GetWorld()->GetTimeSeconds();
}

void ARenetClient::pingServer() {
	const char* CONNECT_PING = "{\"PING\":[]}";
	rnet_send_message(rnet_get_reliable_channel_id(), CONNECT_PING, strlen(CONNECT_PING));
}

// Called every frame
void ARenetClient::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
	const char* CONNECT_REQUEST = "{\"RequestConnect\":[]}";
    const char* CONNECT_RESPONSE = "{\"Connect\":\"Amon\"}";
    const char* CONNECT_PING = "{\"PING\":[]}";
    const char* CONNECT_PONG = "{\"PONG\":[]}";

    uint64_t now = rnet_time_now();
    uint64_t duration = now - lastUpdate;
    lastUpdate = now;

    const uint8_t** reliableMessages = (const uint8_t**)rnet_get_messages(duration, rnet_get_reliable_channel_id());

	if (!rnet_get_result_status()) {
		UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
		return;
	}


	if (reliableMessages != NULL) {	
		for (const uint8_t** msg = reliableMessages; *msg != NULL; msg++) {
			if (strcmp((const char*)*msg, CONNECT_REQUEST) == 0) {
				rnet_send_message(rnet_get_reliable_channel_id(), CONNECT_RESPONSE, strlen(CONNECT_RESPONSE));
				lastCommunication = GetWorld()->GetTimeSeconds();
				UE_LOG(LogTemp, Log, TEXT("Update Login"));

				if (!rnet_get_result_status()) {
					UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
					return;
				}
			}

			else if (strcmp((const char*)*msg, CONNECT_PONG) == 0) {
				lastCommunication = GetWorld()->GetTimeSeconds();

				if (!rnet_get_result_status()) {
					UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
					return;
				}
			}
		}
	}

	if (GetWorld()->GetTimeSeconds() - lastCommunication >= 4.0f) {
		lastCommunication = GetWorld()->GetTimeSeconds();
		pingServer();
	}

    rnet_send_messages();

	if (!rnet_get_result_status()) {
		UE_LOG(LogTemp, Error, TEXT("Rnet client error: %s: %s"), UTF8_TO_TCHAR(rnet_get_result_message()), UTF8_TO_TCHAR(rnet_get_result_detail()));
		return;
	}
}


