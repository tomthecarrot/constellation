using UnityEngine;

[CreateAssetMenu(fileName = "ConstellationBuildSettings", menuName = "ScriptableObjects/ConstellationBuildSettings", order = 1)]
public class ConstellationBuildSettings : ScriptableObject
{
    public string androidPluginsDir; // e.g. "Assets/Plugins/Android"
    public string[] androidLibNames; // e.g. "arm64-v8a/unity_states.so"
}
