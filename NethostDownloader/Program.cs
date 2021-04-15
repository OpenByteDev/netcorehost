using System;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;
using NuGet.Common;
using NuGet.Packaging;
using NuGet.Protocol;
using NuGet.Protocol.Core.Types;

namespace NethostDownloader {
    public static class Program {
        public static async Task Main(DirectoryInfo runtimesDirectory, bool deleteOld = false) {
            if (runtimesDirectory is null)
                runtimesDirectory = new DirectoryInfo("runtimes");

            var osFromPackageTitleRegex = new Regex(@"^runtime\.(\w+(?:-\w+)*)\.");

            var repository = Repository.Factory.GetCoreV3("https://api.nuget.org/v3/index.json");

            var cache = new SourceCacheContext();
            var logger = NullLogger.Instance;
            var cancellationToken = CancellationToken.None;

            var searchResource = await repository.GetResourceAsync<PackageSearchResource>().ConfigureAwait(false);
            var findResource = await repository.GetResourceAsync<FindPackageByIdResource>().ConfigureAwait(false);
            var searchFilter = new SearchFilter(includePrerelease: false);

            var results = await searchResource.SearchAsync(
                "Microsoft.NETCore.DotNetAppHost",
                searchFilter,
                skip: 0,
                take: 50,
                logger,
                cancellationToken).ConfigureAwait(false);

            if (deleteOld && runtimesDirectory.Exists)
                runtimesDirectory.Delete(true);

            runtimesDirectory.Create();

            foreach (var package in results.Where(package => package.Authors == "Microsoft")) {
                var match = osFromPackageTitleRegex.Match(package.Title);
                if (!match.Success)
                    continue;
                var os = match.Groups[1].Value;

                Console.WriteLine(os);
                using var packageStream = new MemoryStream();
                await findResource.CopyNupkgToStreamAsync(
                    package.Identity.Id,
                    package.Identity.Version,
                    packageStream,
                    cache,
                    logger,
                    cancellationToken).ConfigureAwait(false);

                using var packageReader = new PackageArchiveReader(packageStream);
                var files = (await packageReader.GetItemsAsync("runtimes", cancellationToken).ConfigureAwait(false)).FirstOrDefault()?.Items;

                if (files == null)
                    continue;

                var packageDirectory = runtimesDirectory.CreateSubdirectory(os);
                foreach (var file in files) {
                    packageReader.ExtractFile(file, Path.Combine(packageDirectory.FullName, Path.GetFileName(file)), logger);
                }
            }
        }
    }
}
