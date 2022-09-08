using ASTContext = CppSharp.AST.ASTContext;
using Rflct = System.Reflection;
using Console = System.Console;
using Driver = CppSharp.Driver;
using Gen = CppSharp.Generators;
using DirectoryInfo = System.IO.DirectoryInfo;
using Path = System.IO.Path;
using System.Collections.Generic;
using RtInfo = System.Runtime.InteropServices.RuntimeInformation;
using OS = System.Runtime.InteropServices.OSPlatform;
using Exception = System.Exception;


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
        private static DirectoryInfo project_dir = GetProjectDir();
        private readonly LibInfo lib_info;
        private readonly static string DYLIB_EXTENSION =
            (RtInfo.IsOSPlatform(OS.Linux) || RtInfo.IsOSPlatform(OS.FreeBSD)) ? ".so"
            : RtInfo.IsOSPlatform(OS.Windows) ? ".dll"
            // Using ".dylib" on mac silently inhibits DLLImports 🤦‍♂️
            : RtInfo.IsOSPlatform(OS.OSX) ? ""
            : throw new Exception("unknown platform");

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
                    project_dir.FullName, "demos", "unity_states", "cs", "generated", "cpp_sharp"
                ),
                input_dir: Path.Join(project_dir.FullName, "demos", "unity_states", "rust"),
                cargo_artifact_dir: Path.Join(project_dir.FullName, "target", "debug"),
                "unity_states"
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
                CppSharp.ConsoleDriver.Run(new Codegen(lib_info: lib));
            }

            return 0;
        }

        private Codegen(LibInfo lib_info)
        {
            this.lib_info = lib_info;
        }

        /// Setup the driver options here.
        public void Setup(CppSharp.Driver driver)
        {
            var options = driver.Options;
            options.GeneratorKind = Gen.GeneratorKind.CSharp;
            options.OutputDir = this.lib_info.output_dir.FullName;
            options.GenerateClassTemplates = false;
            options.GenerateFinalizers = false;

            // hard coding "unity_states" as a stopgap until we decide which target
            // provides all the symbols for the necessary libraries
            var module = options.AddModule("unity_states");
            module.Libraries.Add($"libunity_states{Codegen.DYLIB_EXTENSION}");
            module.OutputNamespace = this.lib_info.crate_name;

            module.IncludeDirs.Add(this.lib_info.input_dir.FullName);
            module.Headers.Add("generated.h");
            module.LibraryDirs.Add(this.lib_info.cargo_artifact_dir.FullName);
        }

        /// Setup your passes here.
        public void SetupPasses(Driver driver) { }

        /// Do transformations that should happen before passes are processed.
        public void Preprocess(Driver driver, ASTContext ctx) { }

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
            if (project_dir.Name != "client")
            {
                project_dir = project_dir.Parent ?? project_dir.Root;
            }
            // Go up one more to reach the toplevel directory that holds all the projects
            return project_dir.Parent ?? project_dir.Root;
        }
    }


}
