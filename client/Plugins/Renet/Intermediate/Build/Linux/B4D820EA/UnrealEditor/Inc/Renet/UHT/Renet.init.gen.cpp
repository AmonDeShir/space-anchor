// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
PRAGMA_DISABLE_DEPRECATION_WARNINGS
void EmptyLinkFunctionForGeneratedCodeRenet_init() {}
	static FPackageRegistrationInfo Z_Registration_Info_UPackage__Script_Renet;
	FORCENOINLINE UPackage* Z_Construct_UPackage__Script_Renet()
	{
		if (!Z_Registration_Info_UPackage__Script_Renet.OuterSingleton)
		{
			static const UECodeGen_Private::FPackageParams PackageParams = {
				"/Script/Renet",
				nullptr,
				0,
				PKG_CompiledIn | 0x00000000,
				0x12707845,
				0x31A5299E,
				METADATA_PARAMS(nullptr, 0)
			};
			UECodeGen_Private::ConstructUPackage(Z_Registration_Info_UPackage__Script_Renet.OuterSingleton, PackageParams);
		}
		return Z_Registration_Info_UPackage__Script_Renet.OuterSingleton;
	}
	static FRegisterCompiledInInfo Z_CompiledInDeferPackage_UPackage__Script_Renet(Z_Construct_UPackage__Script_Renet, TEXT("/Script/Renet"), Z_Registration_Info_UPackage__Script_Renet, CONSTRUCT_RELOAD_VERSION_INFO(FPackageReloadVersionInfo, 0x12707845, 0x31A5299E));
PRAGMA_ENABLE_DEPRECATION_WARNINGS
