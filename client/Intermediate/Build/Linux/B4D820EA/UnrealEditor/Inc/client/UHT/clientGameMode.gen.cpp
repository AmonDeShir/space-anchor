// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "client/clientGameMode.h"
PRAGMA_DISABLE_DEPRECATION_WARNINGS
void EmptyLinkFunctionForGeneratedCodeclientGameMode() {}
// Cross Module References
	CLIENT_API UClass* Z_Construct_UClass_AclientGameMode();
	CLIENT_API UClass* Z_Construct_UClass_AclientGameMode_NoRegister();
	ENGINE_API UClass* Z_Construct_UClass_AGameModeBase();
	UPackage* Z_Construct_UPackage__Script_client();
// End Cross Module References
	void AclientGameMode::StaticRegisterNativesAclientGameMode()
	{
	}
	IMPLEMENT_CLASS_NO_AUTO_REGISTRATION(AclientGameMode);
	UClass* Z_Construct_UClass_AclientGameMode_NoRegister()
	{
		return AclientGameMode::StaticClass();
	}
	struct Z_Construct_UClass_AclientGameMode_Statics
	{
		static UObject* (*const DependentSingletons[])();
#if WITH_METADATA
		static const UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[];
#endif
		static const FCppClassTypeInfoStatic StaticCppClassTypeInfo;
		static const UECodeGen_Private::FClassParams ClassParams;
	};
	UObject* (*const Z_Construct_UClass_AclientGameMode_Statics::DependentSingletons[])() = {
		(UObject* (*)())Z_Construct_UClass_AGameModeBase,
		(UObject* (*)())Z_Construct_UPackage__Script_client,
	};
#if WITH_METADATA
	const UECodeGen_Private::FMetaDataPairParam Z_Construct_UClass_AclientGameMode_Statics::Class_MetaDataParams[] = {
		{ "HideCategories", "Info Rendering MovementReplication Replication Actor Input Movement Collision Rendering HLOD WorldPartition DataLayers Transformation" },
		{ "IncludePath", "clientGameMode.h" },
		{ "ModuleRelativePath", "clientGameMode.h" },
		{ "ShowCategories", "Input|MouseInput Input|TouchInput" },
	};
#endif
	const FCppClassTypeInfoStatic Z_Construct_UClass_AclientGameMode_Statics::StaticCppClassTypeInfo = {
		TCppClassTypeTraits<AclientGameMode>::IsAbstract,
	};
	const UECodeGen_Private::FClassParams Z_Construct_UClass_AclientGameMode_Statics::ClassParams = {
		&AclientGameMode::StaticClass,
		"Game",
		&StaticCppClassTypeInfo,
		DependentSingletons,
		nullptr,
		nullptr,
		nullptr,
		UE_ARRAY_COUNT(DependentSingletons),
		0,
		0,
		0,
		0x008802ACu,
		METADATA_PARAMS(Z_Construct_UClass_AclientGameMode_Statics::Class_MetaDataParams, UE_ARRAY_COUNT(Z_Construct_UClass_AclientGameMode_Statics::Class_MetaDataParams))
	};
	UClass* Z_Construct_UClass_AclientGameMode()
	{
		if (!Z_Registration_Info_UClass_AclientGameMode.OuterSingleton)
		{
			UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_AclientGameMode.OuterSingleton, Z_Construct_UClass_AclientGameMode_Statics::ClassParams);
		}
		return Z_Registration_Info_UClass_AclientGameMode.OuterSingleton;
	}
	template<> CLIENT_API UClass* StaticClass<AclientGameMode>()
	{
		return AclientGameMode::StaticClass();
	}
	DEFINE_VTABLE_PTR_HELPER_CTOR(AclientGameMode);
	AclientGameMode::~AclientGameMode() {}
	struct Z_CompiledInDeferFile_FID_home_amon_Projects_space_anchor_client_Source_client_clientGameMode_h_Statics
	{
		static const FClassRegisterCompiledInInfo ClassInfo[];
	};
	const FClassRegisterCompiledInInfo Z_CompiledInDeferFile_FID_home_amon_Projects_space_anchor_client_Source_client_clientGameMode_h_Statics::ClassInfo[] = {
		{ Z_Construct_UClass_AclientGameMode, AclientGameMode::StaticClass, TEXT("AclientGameMode"), &Z_Registration_Info_UClass_AclientGameMode, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(AclientGameMode), 4005264092U) },
	};
	static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_home_amon_Projects_space_anchor_client_Source_client_clientGameMode_h_3666049167(TEXT("/Script/client"),
		Z_CompiledInDeferFile_FID_home_amon_Projects_space_anchor_client_Source_client_clientGameMode_h_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_home_amon_Projects_space_anchor_client_Source_client_clientGameMode_h_Statics::ClassInfo),
		nullptr, 0,
		nullptr, 0);
PRAGMA_ENABLE_DEPRECATION_WARNINGS
