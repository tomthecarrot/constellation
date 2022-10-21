using ASTContext = CppSharp.AST.ASTContext;
using Rflct = System.Reflection;
using Console = System.Console;
using Driver = CppSharp.Driver;
using Gen = CppSharp.Generators;
using DirectoryInfo = System.IO.DirectoryInfo;
using Path = System.IO.Path;
using System.Collections.Generic;
using RtInfo = System.Runtime.InteropServices.RuntimeInformation;
using IO = System.IO;
using OS = System.Runtime.InteropServices.OSPlatform;
using Exception = System.Exception;
using CppSharp.AST;
using CppSharp.Passes;


namespace Codegen
{

    public record LibInfo
    {
        public DirectoryInfo output_dir { get; init; }
        public DirectoryInfo input_dir { get; init; }
        public DirectoryInfo cargo_artifact_dir { get; init; }
        public string crate_name { get; init; }

        public LibInfo(string output_dir, string input_dir, string cargo_artifact_dir, string crate_name)
        {
            this.output_dir = new DirectoryInfo(output_dir);
            this.input_dir = new DirectoryInfo(input_dir);
            this.cargo_artifact_dir = new DirectoryInfo(cargo_artifact_dir);
            this.crate_name = crate_name;
        }
    }

    public sealed class Codegen : CppSharp.ILibrary
    {
        private const string ENV_VAR_NAME_DLLIMPORT_OVERRIDE = "CONSTELLATION_DLLIMPORT_NAME";

        private static DirectoryInfo project_dir = GetProjectDir();
        private readonly LibInfo lib_info;
        private string override_lib_name;

        static int Main(string[] args)
        {
            const bool OVERWRITE = true;

            if (args.Length > 0)
            {
                Console.Error.WriteLine("Too many arguments!");
                return -1;
            }

            // Configure information about the libs we will be generating
            var libs = new List<LibInfo>();
            libs.Add(new LibInfo(
                output_dir: Path.Join(
                    project_dir.FullName, "client", "cs", "src", "generated", "cpp_sharp"
                ),
                input_dir: Path.Join(project_dir.FullName, "client", "rust"),
                cargo_artifact_dir: Path.Join(project_dir.FullName, "target", "debug"),
                "tp_client"
            ));
            libs.Add(new LibInfo(
                output_dir: Path.Join(
                    project_dir.FullName, "demos", "unity_states", "cs", "src", "generated", "cpp_sharp"
                ),
                input_dir: Path.Join(project_dir.FullName, "demos", "unity_states", "rust"),
                cargo_artifact_dir: Path.Join(project_dir.FullName, "target", "debug"),
                "unity_states"
            ));
            libs.Add(new LibInfo(
                output_dir: Path.Join(
                    project_dir.FullName, "client", "contract_example", "cs", "src", "generated", "cpp_sharp"
                ),
                input_dir: Path.Join(project_dir.FullName, "client", "contract_example", "rust"),
                cargo_artifact_dir: Path.Join(project_dir.FullName, "target", "debug"),
                "tp_contract_example"
            ));
            Console.WriteLine($"Project Directory: {project_dir}");

            foreach (var lib in libs)
            {
                Console.WriteLine($"Generating c# pinvoke for {lib.crate_name}");

                // Handle potentially overwriting existing `output_dir`
                var has_files = lib.output_dir.Exists && (
                    lib.output_dir.GetDirectories().Length != 0
                    || lib.output_dir.GetFiles().Length != 0
                );
                if (!OVERWRITE && has_files)
                {
                    Console.Error.WriteLine("Output directory must be empty.");
                    return -1;
                }
                else if (OVERWRITE && has_files)
                {
                    lib.output_dir.Delete(true);
                }

                // Actually generate the code
                var override_lib_name = System.Environment.GetEnvironmentVariable(ENV_VAR_NAME_DLLIMPORT_OVERRIDE);
                CppSharp.ConsoleDriver.Run(new Codegen(lib, override_lib_name ?? "unity_states"));
            }

            return 0;
        }

        public Codegen(LibInfo lib_info, string override_lib_name)
        {
            this.lib_info = lib_info;
            this.override_lib_name = override_lib_name;
        }

        /// Setup the driver options here.
        public void Setup(CppSharp.Driver driver)
        {
            // Copy all DLLs to override location.
            //
            // Leaf `csproj`s that contain their own native libraries and depend on this base `csproj` need to
            // be able to override all `DllImport` arguments at compile time to one single library name.
            // This is because the base `csproj` exposes client functionality such as `Baseline` and `RBox` to C#,
            // while a leaf `csproj` may expose its own C# FFI such as custom contracts.
            //
            // The Constellation build process in Rust (`cargo build` step) currently outputs a monolithic native library,
            // so all symbols from both the leaf crate AND the base crate (tp_client + rsharp) are present.
            // If C# code `DllImport`s from multiple libraries, then references to base and leaf functionality from C#
            // (e.g. `Baseline` and `MyCustomContract`, respectively) will render unusable because the objects
            // do not coexist in the same allocation of memory made for each library on `dlopen`.
            //
            // CppSharp uses the native library's file name as the argument for `DllImport()`.
            // (Reference: https://github.com/mono/CppSharp/blob/main/src/Generator/Generators/CSharp/CSharpSources.cs#L3497).
            // Since there is no parameter we've found in CppSharp to do this override, we copy the base library to
            // a destination file with the name provided by the environment variable defined in `Codegen.ENV_VAR_NAME_DLLIMPORT_OVERRIDE`.
            // This "tricks" CppSharp into generating the same symbols in C# while using the overriding name.
            //
            // CppSharp codegen is the only process in which this destination file is used.
            // After this invocation of CppSharp codegen, the copy operation does not affect subsequent compile steps.

            // No need to copy if the output name is the same as the input
            if (this.lib_info.crate_name != this.override_lib_name)
            {
                var directoryInfo = new DirectoryInfo(this.lib_info.cargo_artifact_dir.FullName);
                var filesList = directoryInfo.GetFiles($"lib{this.lib_info.crate_name}.*");
                foreach (var fileInfo in filesList)
                {
                    var lib_ext = fileInfo.Name.Split('.')[1];

                    // Example: Copy `libtp_client.so` -> `libyolo.so`
                    IO.File.Copy(
                        $"{this.lib_info.cargo_artifact_dir.FullName}/lib{this.lib_info.crate_name}.{lib_ext}",
                        $"{this.lib_info.cargo_artifact_dir.FullName}/lib{this.override_lib_name}.{lib_ext}",
                        true // overwrite destination file if it already exists
                    );
                }
            }

            var options = driver.Options;
            options.GeneratorKind = Gen.GeneratorKind.CSharp;
            options.OutputDir = this.lib_info.output_dir.FullName;
            options.GenerateClassTemplates = false;
            options.GenerateFinalizers = false;

#if UNITY_IOS
            // This option is only used for the iOS DLL. It replaces the argument
            // in all `DllImport`s with `__Internal` for static linking.
            options.GenerateInternalImports = true;
#endif

            var module = options.AddModule(this.override_lib_name);
            module.Libraries.Add($"lib{this.override_lib_name}");
            module.OutputNamespace = this.lib_info.crate_name;

            module.IncludeDirs.Add(this.lib_info.input_dir.FullName);
            module.Headers.Add("generated.h");
            module.LibraryDirs.Add(this.lib_info.cargo_artifact_dir.FullName);
        }

        /// Setup your passes here.
        public void SetupPasses(Driver driver) { }

        /// Do transformations that should happen before passes are processed.
        public void Preprocess(Driver driver, ASTContext ctx)
        {
            new VisitMethodPass_IgnoreCopyConstructor().VisitASTContext(ctx);
        }

        /// Do transformations that should happen after passes are processed.
        public void Postprocess(Driver driver, ASTContext ctx) { }

        /// Get the toplevel folder in the project
        public static DirectoryInfo GetProjectDir()
        {
            // If we are compiling for a target different from the native platform,
            // this will be nested one folder deeper (to disambiguate the platform).
            // So this code will check two folders and pick one based on the expected
            // folder layout.

            var assembly_file = new DirectoryInfo(Rflct.Assembly.GetExecutingAssembly().Location);

            DirectoryInfo project_dir = assembly_file;
            const uint n_steps_up = 5;
            for (var i = 0; i < n_steps_up; i++)
            {
                project_dir = project_dir.Parent ?? project_dir.Root;
            }

            // We are now either at the `client` folder or one folder deeper if
            // compiling to a non-native architecture. Lets figure out which one.
            // Yes, this is a dirty hack.
            // The "client" directory comes from this repo (Constellation).
            // The "codegen_movieoke" directory is added temporarily to support codegen
            // in the Movieoke repo, which references this codegen csproj.
            if (project_dir.Name != "client" && project_dir.Name != "codegen_movieoke")
            {
                project_dir = project_dir.Parent ?? project_dir.Root;
            }
            // Go up one more to reach the toplevel directory that holds all the projects
            return project_dir.Parent ?? project_dir.Root;
        }
    }


    /// This pass instructs CppSharp to skip copy constructors, which are not
    /// supported since we are compiling C dylibs and static libs (not C++ ones).
    public class VisitMethodPass_IgnoreCopyConstructor : TranslationUnitPass
    {
        public override bool VisitMethodDecl(Method method)
        {
            if (!base.VisitMethodDecl(method))
            {
                return false;
            }

            if (method.IsCopyConstructor)
            {
                method.ExplicitlyIgnore();
            }

            return true;
        }
    }

}
