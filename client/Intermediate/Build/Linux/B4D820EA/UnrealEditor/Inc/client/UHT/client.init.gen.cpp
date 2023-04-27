// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
PRAGMA_DISABLE_DEPRECATION_WARNINGS
void EmptyLinkFunctionForGeneratedCodeclient_init() {}
	static FPackageRegistrationInfo Z_Registration_Info_UPackage__Script_client;
	FORCENOINLINE UPackage* Z_Construct_UPackage__Script_client()
	{
		if (!Z_Registration_Info_UPackage__Script_client.OuterSingleton)
		{
			static const UECodeGen_Private::FPackageParams PackageParams = {
				"/Script/client",
				nullptr,
				0,
				PKG_CompiledIn | 0x00000000,
				0x119CD7CB,
				0x2F7D3C4D,
				METADATA_PARAMS(nullptr, 0)
			};
			UECodeGen_Private::ConstructUPackage(Z_Registration_Info_UPackage__Script_client.OuterSingleton, PackageParams);
		}
		return Z_Registration_Info_UPackage__Script_client.OuterSingleton;
	}
	static FRegisterCompiledInInfo Z_CompiledInDeferPackage_UPackage__Script_client(Z_Construct_UPackage__Script_client, TEXT("/Script/client"), Z_Registration_Info_UPackage__Script_client, CONSTRUCT_RELOAD_VERSION_INFO(FPackageReloadVersionInfo, 0x119CD7CB, 0x2F7D3C4D));
PRAGMA_ENABLE_DEPRECATION_WARNINGS
