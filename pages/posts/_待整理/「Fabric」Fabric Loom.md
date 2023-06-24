# Fabric Loom

**Fabric Loom** 是一个 **Gradle** 插件，提供了如下的 **tasks**：

| 任务名            | 解释                                                         |
| ----------------- | ------------------------------------------------------------ |
| `migrateMapping`  | Migrates the current source to the specified mappings. See [migratemappings](https://fabricmc.net/wiki/tutorial:migratemappings). |
| `remapJar`        | Produces a jar containing the remapped output of the `jar` task. Also appends any included mods for jar-in-jar. Called when running `build`. |
| **`genSources`**  | **使用默认的反编译器（CFR）反编译 minecraft jar**            |
| `downloadAssets`  | Downloads the asset index and asset objects for the configured version of Minecraft into the user cache. |
| `genEclipseRuns`  | Installs Eclipse run configurations and creates the run directory if it does not already exist. |
| `vscode`          | Generates or overwrites a Visual Studio Code `launch.json` file with launch configurations in the `.vscode` directory and creates the run directory if it does not already exist. |
| `remapSourcesJar` | Only exists if an AbstractArchiveTask `sourcesJar` exists. Remaps the output of the `sourcesJar` task in place. |
| **`runClient`**   | **A JavaExec task to launch Fabric Loader as a Minecraft client.**（顾名思义） |
| **`runServer`**   | **A JavaExec task to launch Fabric Loader as a Minecraft dedicated server.**（顾名思义） |

## Dependencies 选项

| 名          | 解释                                                         |
| ----------- | ------------------------------------------------------------ |
| `minecraft` | 定义在开发环境中使用的 Minecraft 版本                        |
| `mappings`  | 定义在开发环境中使用的 mappings                              |
| `include`   | Declares a dependency that should be included as a jar-in-jar in the `remapJar` output. This dependency configuration is not transitive. For non-mod dependencies, Loom will generate a mod JAR with a fabric.mod.json using the name as the mod ID and the same version. |

除了这三个还有  `modCompile`, `modImplementation`, `modApi` 和 `modRuntime` 分别对应 `compile`, `implementation`, `api` 和 `runtime` 用来配置mod依赖的版本