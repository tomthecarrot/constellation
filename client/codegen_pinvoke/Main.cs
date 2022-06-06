using ASTContext = CppSharp.AST.ASTContext;
using Rflct = System.Reflection;
using Console = System.Console;
using Driver = CppSharp.Driver;
using Gen = CppSharp.Generators;
using E = System.Linq.Enumerable;
using DirectoryInfo = System.IO.DirectoryInfo;
using Path = System.IO.Path;
using Dir = System.IO.Directory;


namespace Codegen
{

    public class Codegen : CppSharp.ILibrary
    {
        static DirectoryInfo project_dir = GetProjectDir();
        DirectoryInfo output_dir;

        static int Main(string[] args)
        {
            string? arg0 = null;
            if (args.Length > 1)
            {
                Console.Error.WriteLine("Too many arguments!");
                return -1;
            }
            else if (args.Length == 1)
            {
                arg0 = args[0];
            }

            DirectoryInfo output_dir;
            if (arg0 == null)
            {
                output_dir = new DirectoryInfo(Path.Join(project_dir.FullName, "client", "cs", "src", "generated", "cpp_sharp"));
            }
            else
            {
                output_dir = new DirectoryInfo(arg0);
            }

            Console.WriteLine($"Project Directory: {project_dir}");
            Console.WriteLine($"Output Directory: {output_dir}");
            if (output_dir.Exists && (output_dir.GetDirectories().Length != 0 || output_dir.GetFiles().Length != 0))
            {
                Console.Error.WriteLine("Output directory must be empty.");
                return -1;
            }

            CppSharp.ConsoleDriver.Run(new Codegen(output_dir));

            return 0;
        }

        public Codegen(DirectoryInfo output_dir)
        {
            this.output_dir = output_dir;
        }

        /// Setup the driver options here.
        public void Setup(CppSharp.Driver driver)
        {

            var cargo_artifact_dir = Path.Join(project_dir.FullName, "target", "debug");

            var options = driver.Options;
            options.GeneratorKind = Gen.GeneratorKind.CSharp;
            options.OutputDir = this.output_dir.FullName;

            var module = options.AddModule("tp_client");
            module.IncludeDirs.Add(Path.Join(project_dir.FullName, "client", "rust"));
            module.Headers.Add("generated.h");
            module.LibraryDirs.Add(cargo_artifact_dir);
            module.Libraries.Add("libtp_client.so");
            // module.Undefines.Add("__cplusplus");
        }

        /// Setup your passes here.
        public void SetupPasses(Driver driver) { }

        /// Do transformations that should happen before passes are processed.
        public void Preprocess(Driver driver, ASTContext ctx) { }

        /// Do transformations that should happen after passes are processed.
        public void Postprocess(Driver driver, ASTContext ctx) { }

        /// Get the toplevel folder in the project
        static DirectoryInfo GetProjectDir()
        {
            var assembly_file = new DirectoryInfo(Rflct.Assembly.GetExecutingAssembly().Location);

            DirectoryInfo project_dir = assembly_file;
            const uint n_steps_up = 6;
            for (var i = 0; i < n_steps_up; i++)
            {
                project_dir = project_dir.Parent ?? project_dir.Root;
            }

            return project_dir;
        }
    }


}
