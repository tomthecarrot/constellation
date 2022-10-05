using UnityEditor;
using UnityEditor.Android;
using UnityEditor.Build;
using UnityEditor.Build.Reporting;
using UnityEngine;

class ConstellationBuildPreprocessor : IPostGenerateGradleAndroidProject
{
    private const string pluginsDir = "Assets/Plugins/Constellation/Android";
    private string[] libNames = new string[]
    {
        "arm64-v8a/libunity_states.so",
        "armeabi-v7a/libunity_states.so",
    };

    public int callbackOrder { get { return 0; } }
    public void OnPostGenerateGradleAndroidProject(string projectPath)
    {
        string libDir = $"{projectPath}/src/main/jniLibs";
        foreach (string libName in libNames)
        {
            var src = $"{pluginsDir}/{libName}";
            var dst = $"{libDir}/{libName}";
            FileUtil.DeleteFileOrDirectory(dst);
            FileUtil.CopyFileOrDirectoryFollowSymlinks(src, dst);
        }
    }
}
