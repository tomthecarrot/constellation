using ASTContext = CppSharp.AST.ASTContext;
using Rflct = System.Reflection;
using Console = System.Console;
using Driver = CppSharp.Driver;
using Gen = CppSharp.Generators;
using E = System.Linq.Enumerable;
using DirectoryInfo = System.IO.DirectoryInfo;
using Path = System.IO.Path;
using Dir = System.IO.Directory;
using System.Collections.Generic;


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

    public class Codegen : CppSharp.ILibrary
    {
        private static DirectoryInfo project_dir = GetProjectDir();
        private LibInfo lib_info;
        private const bool overwrite = true;

        static int Main(string[] args)
        {
            if (args.Length > 0)
            {
                Console.Error.WriteLine("Too many arguments!");
                return -1;
            }

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

                if (!Codegen.overwrite && (lib.output_dir.GetDirectories().Length != 0 || lib.output_dir.GetFiles().Length != 0))
                {
                    Console.Error.WriteLine("Output directory must be empty.");
                    return -1;
                }

                lib.output_dir.Delete(true);

                CppSharp.ConsoleDriver.Run(new Codegen(lib));
            }

            return 0;
        }

        public Codegen(LibInfo lib_info)
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

            var module = options.AddModule(this.lib_info.crate_name);
            module.IncludeDirs.Add(this.lib_info.input_dir.FullName);
            module.Headers.Add("generated.h");
            module.LibraryDirs.Add(this.lib_info.cargo_artifact_dir.FullName);
            module.Libraries.Add($"lib{this.lib_info.crate_name}.so"); // macOS: remove .so extension.
            // module.Undefines.Add("__cplusplus");
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
            var assembly_file = new DirectoryInfo(Rflct.Assembly.GetExecutingAssembly().Location);

            DirectoryInfo project_dir = assembly_file;
            const uint n_steps_up = 6; // macOS: 7 steps up.
            for (var i = 0; i < n_steps_up; i++)
            {
                project_dir = project_dir.Parent ?? project_dir.Root;
            }

            return project_dir;
        }
    }


}
