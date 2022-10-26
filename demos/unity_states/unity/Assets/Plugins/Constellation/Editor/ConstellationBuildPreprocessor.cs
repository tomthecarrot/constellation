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
        var buildSettings = ConstellationBuildSettings.GetDefaults();
        string libDir = $"{unityProjectPath}/src/main/jniLibs";
        foreach (string libName in buildSettings.androidLibNames)
        {
            var src = $"{buildSettings.androidPluginsDir}/{libName}";
            var dst = $"{libDir}/{libName}";
            FileUtil.DeleteFileOrDirectory(dst);
            FileUtil.CopyFileOrDirectoryFollowSymlinks(src, dst);
        }
    }
}
