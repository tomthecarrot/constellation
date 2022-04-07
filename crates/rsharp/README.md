# rsharp
`rsharp` is a library that enable good language interoperability between Rust and C#!

## .NET and Cross Platform Support
We target .NET Standard 2.0, as this has common compatibility between all platforms, and
is supported by Unity. When Unity adds .NET 5 support or higher, we will switch to that
platform, as it is the One True™ C# version going forward.

## Using the library
To ensure that `rsharp` can be consumed by as much of the C# ecosystem as possible, we
distribute the code with NuGet. You can use the
[`dotnet` CLI tool](https://docs.microsoft.com/en-us/nuget/consume-packages/install-use-packages-dotnet-cli)
to add it to your project, or through Visual Studio.

### Usage in Unity
Use of `rsharp` in Unity can be easily accomplished with
[NuGetForUnity](https://github.com/GlitchEnzo/NuGetForUnity). We distribute `rsharp`
using NuGet because it simplifies our devops and allows for the library to be developed
without pulling in an entire game engine.

## Project Structrure
The teleportal platform codebase is a monorepo, where each logically separate library
and app is contained in a different folder, with parent folders for any logical
groupings. The `rsharp` library follows this convention, so its folder is located
at a logical spot in the codebase. Right now, nothing else is related to the rsharp
library, so it is located at the toplevel.

From there, we have a folder for each language's code, that follows the conventions of
that language, so `rsharp/rust` is a rust crate, `rsharp/cs` is a .NET package.
