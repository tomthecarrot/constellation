using UnityEngine;

// Attach to Ball prefab.
[RequireComponent(typeof(MeshRenderer))]
public class BallStateDataSourceUnity : MonoBehaviour, IBallStateDataSource
{

    private MeshRenderer meshRenderer;

    void Awake()
    {
        this.meshRenderer = GetComponent<MeshRenderer>();

        // TODO[SER-383]
    }

    void Update()
    {
        // TODO[SER-383]
        LogCurrentData();
    }

    public void LogCurrentData()
    {
        string pos = $"({this.pos_x.ToString()}, {this.pos_y.ToString()}, {this.pos_z.ToString()})";
        string euler = $"({this.euler_x.ToString()}, {this.euler_y.ToString()}, {this.euler_z.ToString()})";
        string scale = $"({this.scale_x.ToString()}, {this.scale_y.ToString()}, {this.scale_z.ToString()})";
        string color = $"{this.color.ToString()}";

        string debug_str = $"{pos}\n{euler}\n{scale}\n{color}";
        Debug.Log(debug_str);
    }

    public BallStateDataSourceType type
    {
        get
        {
            return BallStateDataSourceType.UNITY;
        }
    }

    public float pos_x
    {
        get
        {
            return this.transform.position.x;
        }
    }

    public float pos_y
    {
        get
        {
            return this.transform.position.y;
        }
    }

    public float pos_z
    {
        get
        {
            return this.transform.position.z;
        }
    }

    public short euler_x
    {
        get
        {
            return (short)this.transform.eulerAngles.x;
        }
    }

    public short euler_y
    {
        get
        {
            return (short)this.transform.eulerAngles.y;
        }
    }

    public short euler_z
    {
        get
        {
            return (short)this.transform.eulerAngles.z;
        }
    }

    public float scale_x
    {
        get
        {
            return this.transform.localScale.x;
        }
    }

    public float scale_y
    {
        get
        {
            return this.transform.localScale.y;
        }
    }

    public float scale_z
    {
        get
        {
            return this.transform.localScale.z;
        }
    }

    public ulong color
    {
        get
        {
            // Convert to 16-bit raw RGBA
            UnityEngine.Color c = this.meshRenderer.materials[0].color;
            ushort r = (ushort)(c.r * 65535);
            ushort g = (ushort)(c.g * 65535);
            ushort b = (ushort)(c.b * 65535);
            ushort a = (ushort)(c.a * 65535);

            ulong rgba = (ulong)((r << 48) | (g << 32) | (b << 16) | a);
            return rgba;
        }
    }
}
