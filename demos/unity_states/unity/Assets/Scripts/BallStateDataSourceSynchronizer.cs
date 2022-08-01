using UnityEngine;
using TP = Teleportal.Client;
using States = Teleportal.Client.Contract.Properties.States;
using RSharp;

public class BallStateDataSourceSynchronizer : MonoBehaviour
{
    [SerializeField] private BallStateDataSourcePlatform srcPlatform;
    [SerializeField] private BallStateDataSourceUnity srcUnity;
    private bool readyToSynchronize = false;

    void Awake()
    {
        if ((null != this.srcPlatform) && (null != this.srcUnity))
        {
            this.readyToSynchronize = true;
        }
        else
        {
            UnityEngine.Debug.LogError("Platform or Unity data source is not assigned!");
        }
    }

    void Update()
    {
        // Apply Unity state to Platform baseline.
        if (this.readyToSynchronize)
        {
            this.srcPlatform.pos_x = this.srcUnity.pos_x;
            this.srcPlatform.pos_y = this.srcUnity.pos_y;
            this.srcPlatform.pos_z = this.srcUnity.pos_z;
            this.srcPlatform.euler_x = this.srcUnity.euler_x;
            this.srcPlatform.euler_y = this.srcUnity.euler_y;
            this.srcPlatform.euler_z = this.srcUnity.euler_z;
            this.srcPlatform.scale_x = this.srcUnity.scale_x;
            this.srcPlatform.scale_y = this.srcUnity.scale_y;
            this.srcPlatform.scale_z = this.srcUnity.scale_z;
            this.srcPlatform.color = this.srcUnity.color;
        }
    }
}
