using UnityEngine;
using UnityEngine.UI;

public class BallStateBillboardViewer : MonoBehaviour
{
    [SerializeField] private bool isPlatform;
    [SerializeField] private BallStateDataSourcePlatform dataSourcePlatform;
    [SerializeField] private BallStateDataSourceUnity dataSourceUnity;
    [SerializeField] private Text typeView;
    [SerializeField] private Text detailsView;

    private IBallStateDataSource dataSource;

    void Awake()
    {
        this.typeView.text = this.isPlatform ? "PLATFORM" : "UNITY";
        this.dataSource = this.isPlatform ? this.dataSourcePlatform : this.dataSourceUnity;
    }

    void Update()
    {
        string pos = $"X: {this.dataSource.pos_x}\nY: {this.dataSource.pos_y}\nZ: {this.dataSource.pos_z}";
        string euler = $"θX: {this.dataSource.euler_x}\nθY: {this.dataSource.euler_y}\nθZ: {this.dataSource.euler_z}";
        string scale = $"X: {this.dataSource.scale_x}\nY: {this.dataSource.scale_y}\nZ: {this.dataSource.scale_z}";
        string color = $"C: {this.dataSource.color}";
        string detailsStr = $"{pos}\n{euler}\n{scale}\n{color}";
        this.detailsView.text = detailsStr;
    }
}
