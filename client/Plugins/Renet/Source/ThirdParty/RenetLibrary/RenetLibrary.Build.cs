using System.IO;
using UnrealBuildTool;

public class RenetLibrary : ModuleRules
{
	public RenetLibrary(ReadOnlyTargetRules Target) : base(Target)
	{
		Type = ModuleType.External;

		if (Target.Platform == UnrealTargetPlatform.Win64)
		{
			// Add the import library
			PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "x64", "Release", "RenetLibrary.lib"));

			// Delay-load the DLL, so we can load it from the right place first
			PublicDelayLoadDLLs.Add("RenetLibrary.dll");

			// Ensure that the DLL is staged along with the executable
			RuntimeDependencies.Add("$(PluginDir)/Binaries/ThirdParty/RenetLibrary/Win64/RenetLibrary.dll");
        }
        // else if (Target.Platform == UnrealTargetPlatform.Mac)
        // {
        //     PublicDelayLoadDLLs.Add(Path.Combine(ModuleDirectory, "Mac", "Release", "libExampleLibrary.dylib"));
        //     RuntimeDependencies.Add("$(PluginDir)/Source/ThirdParty/RenetLibrary/Mac/Release/libExampleLibrary.dylib");
        // }
        else if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			string ExampleSoPath = Path.Combine("$(PluginDir)", "Binaries", "ThirdParty", "RenetLibrary", "Linux", "x86_64-unknown-linux-gnu", "RenetLibrary.so");
			PublicAdditionalLibraries.Add(ExampleSoPath);
			PublicDelayLoadDLLs.Add(ExampleSoPath);
			RuntimeDependencies.Add(ExampleSoPath);
		}
	}
}