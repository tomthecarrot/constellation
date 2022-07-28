using UnityEngine;
using TP = Teleportal.Client;
using States = Teleportal.Client.Contract.Properties.States;
using RSharp;

public class BallStateDataSourcePlatform : MonoBehaviour, IBallStateDataSource
{
    // Platform interop types
    private TP.Baseline baselineMain;
    private BallContract ballContract;
    private TP.Object.ObjectHandle ballObject;

    private States.StateHandle_F32 stateHandlePosX;
    private States.StateHandle_F32 stateHandlePosY;
    private States.StateHandle_F32 stateHandlePosZ;
    private States.StateHandle_I16 stateHandleEulerX;
    private States.StateHandle_I16 stateHandleEulerY;
    private States.StateHandle_I16 stateHandleEulerZ;
    private States.StateHandle_F32 stateHandleScaleX;
    private States.StateHandle_F32 stateHandleScaleY;
    private States.StateHandle_F32 stateHandleScaleZ;
    private States.StateHandle_U64 stateHandleColor;

    void Awake()
    {
        InstantiatePlatformObject();
    }

    void Update()
    {
        LogCurrentData();
    }

    private void InstantiatePlatformObject()
    {
        this.baselineMain = new TP.Baseline(true);
        this.ballContract = BallContract.Register(this.baselineMain);
        this.ballObject = this.ballContract.ObjectCreate(
            this.baselineMain,
            0f, 0f, 0f,
            0, 0, 0,
            0f, 0f, 0f,
            0
        );

        ConfigurePlatformObjectStates();
    }

    private void ConfigurePlatformObjectStates()
    {
        this.stateHandlePosX = this.baselineMain.BindStateF32(this.ballContract.States.PosX, this.ballObject);
        this.stateHandlePosY = this.baselineMain.BindStateF32(this.ballContract.States.PosY, this.ballObject);
        this.stateHandlePosZ = this.baselineMain.BindStateF32(this.ballContract.States.PosZ, this.ballObject);
        this.stateHandleEulerX = this.baselineMain.BindStateI16(this.ballContract.States.EulerX, this.ballObject);
        this.stateHandleEulerY = this.baselineMain.BindStateI16(this.ballContract.States.EulerY, this.ballObject);
        this.stateHandleEulerZ = this.baselineMain.BindStateI16(this.ballContract.States.EulerZ, this.ballObject);
        this.stateHandleScaleX = this.baselineMain.BindStateF32(this.ballContract.States.ScaleX, this.ballObject);
        this.stateHandleScaleY = this.baselineMain.BindStateF32(this.ballContract.States.ScaleY, this.ballObject);
        this.stateHandleScaleZ = this.baselineMain.BindStateF32(this.ballContract.States.ScaleZ, this.ballObject);
        this.stateHandleColor = this.baselineMain.BindStateU64(this.ballContract.States.Color, this.ballObject);
    }

    public void LogCurrentData()
    {
        string type = this.type.ToString();
        string pos = $"({this.pos_x.ToString()}, {this.pos_y.ToString()}, {this.pos_z.ToString()})";
        string euler = $"({this.euler_x.ToString()}, {this.euler_y.ToString()}, {this.euler_z.ToString()})";
        string scale = $"({this.scale_x.ToString()}, {this.scale_y.ToString()}, {this.scale_z.ToString()})";
        string color = $"{this.color.ToString()}";

        string debug_str = $"{type}:\n{pos}\n{euler}\n{scale}\n{color}";
        Debug.Log(debug_str);
    }

    public BallStateDataSourceType type
    {
        get
        {
            return BallStateDataSourceType.PLATFORM;
        }
    }

    public float pos_x
    {
        get
        {
            return this.baselineMain.State(this.stateHandlePosX).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandlePosX).Value = new RSharp.RBox_F32(value);
        }
    }

    public float pos_y
    {
        get
        {
            return this.baselineMain.State(this.stateHandlePosY).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandlePosY).Value = new RSharp.RBox_F32(value);
        }
    }

    public float pos_z
    {
        get
        {
            return this.baselineMain.State(this.stateHandlePosZ).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandlePosZ).Value = new RSharp.RBox_F32(value);
        }
    }

    public short euler_x
    {
        get
        {
            return this.baselineMain.State(this.stateHandleEulerX).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleEulerX).Value = new RSharp.RBox_I16(value);
        }
    }

    public short euler_y
    {
        get
        {
            return this.baselineMain.State(this.stateHandleEulerY).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleEulerY).Value = new RSharp.RBox_I16(value);
        }
    }

    public short euler_z
    {
        get
        {
            return this.baselineMain.State(this.stateHandleEulerZ).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleEulerZ).Value = new RSharp.RBox_I16(value);
        }
    }

    public float scale_x
    {
        get
        {
            return this.baselineMain.State(this.stateHandleScaleX).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleScaleX).Value = new RSharp.RBox_F32(value);
        }
    }

    public float scale_y
    {
        get
        {
            return this.baselineMain.State(this.stateHandleScaleY).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleScaleY).Value = new RSharp.RBox_F32(value);
        }
    }

    public float scale_z
    {
        get
        {
            return this.baselineMain.State(this.stateHandleScaleZ).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleScaleZ).Value = new RSharp.RBox_F32(value);
        }
    }

    public ulong color
    {
        get
        {
            return this.baselineMain.State(this.stateHandleColor).Value.Value;
        }
        set
        {
            this.baselineMain.State(this.stateHandleColor).Value = new RSharp.RBox_U64(value);
        }
    }
}
