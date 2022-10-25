using System.Linq;
using UnityEngine;
using UnityEditor;

[CreateAssetMenu(fileName = "ConstellationBuildSettings", menuName = "Teleportal/Constellation Build Settings", order = 1)]
public class ConstellationBuildSettings : ScriptableObject
{
    public string dotnetProjectPath; // e.g. "Platform/cs/src"
    public string androidPluginsDir; // e.g. "Assets/Plugins/Android"
    public string[] androidLibNames; // e.g. "arm64-v8a/unity_states.so"

    public static ConstellationBuildSettings GetDefaults()
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

    private const string ERR_MSG_MISSING_SETTINGS = "Missing build settings. Create one via Assets menu > Create > Teleportal > Constellation Build Settings and configure it.";
    private const string ERR_MSG_TOO_MANY_SETTINGS = "There are more than one ConstellationBuildSettings objects in the project. Remove all except one and check the configuration.";
}
