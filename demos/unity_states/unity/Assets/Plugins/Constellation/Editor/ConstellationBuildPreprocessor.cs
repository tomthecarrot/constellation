using System.Linq;
using UnityEditor;
using UnityEditor.Android;
using UnityEditor.Build;
using UnityEditor.Build.Reporting;
using UnityEngine;

/// Unity's Android build process copies symbolic links, not their target files.
/// This preprocessor ensures that the gradle project has access to Constellation
/// native libraries by de-symbolicating their links (copying the target files).
public class ConstellationBuildPreprocessor : IPostGenerateGradleAndroidProject
{
    public int callbackOrder { get { return 0; } }
    public void OnPostGenerateGradleAndroidProject(string unityProjectPath)
    {
        var buildSettings = GetSettings();
        string libDir = $"{unityProjectPath}/src/main/jniLibs";
        foreach (string libName in buildSettings.androidLibNames)
        {
            var src = $"{buildSettings.androidPluginsDir}/{libName}";
            var dst = $"{libDir}/{libName}";
            FileUtil.DeleteFileOrDirectory(dst);
            FileUtil.CopyFileOrDirectoryFollowSymlinks(src, dst);
        }
    }

    private ConstellationBuildSettings GetSettings()
    {
        var settingsObjects = AssetDatabase.FindAssets("t:ConstellationBuildSettings").ToList()
            .Select(AssetDatabase.GUIDToAssetPath)
            .Select(AssetDatabase.LoadAssetAtPath<ConstellationBuildSettings>)
            .ToList();

        if (settingsObjects.Count == 0)
        {
            Debug.LogError($"[Constellation] {ERR_MSG_MISSING_SETTINGS}");
            return null;
        }
        else if (settingsObjects.Count > 1)
        {
            Debug.LogError($"[Constellation] {ERR_MSG_TOO_MANY_SETTINGS}");
            return null;
        }
        return settingsObjects[0];
    }

    private const string ERR_MSG_MISSING_SETTINGS = "Missing build settings! Create one via Assets menu > Create > ScriptableObjects > ConstellationBuildSettings and configure it.";
    private const string ERR_MSG_TOO_MANY_SETTINGS = "There are more than one ConstellationBuildSettings objects in the project! Remove all except one and check the configuration.";
}
