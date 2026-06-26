type BuildSystem = "autotools"

schema BuildStep:
    '''
    构建步骤

    Attributes
    ----------
    script: bash 脚本
    env: bash 环境变量

    Examples
    --------
    ```kcl
    {
        script: """
            echo $message
            some bash command
        """
        env: {
            message: ""
        }
    }
    ```
    '''
    script: str
    env: {str:str}

type SourceType = "Remote" | "Local" | "Patch"
type ChecksumType = "sha256"

schema Source:
    """
    外部源文件

    Attributes
    ----------
    type: 源文件类型
    url: 统一资源描述符
    checksum: 校验码
    checksumType: 校验码类型
    """
    type: SourceType
    url: str
    checksum: str
    checksumType?: ChecksumType = "sha256"

schema Output:
    summary: str
    description: str
    runtimeRequires?: [str]
    buildRequires?: [str]
    files?: [str]

schema Package:
    name: str
    version: str
    release: str
    vcs: str
    license: str
    url: str
    # `output` 是一个键为任意字符串的表，其中 `main` 键必须，作为主包的对应部分。
    outputs?: {str:Output}
    # `buildSystem` 是对构建系统的声明，用于进一步向 buildSteps 注入默认内容。
    buildSystem?: BuildSystem
    # 见 `schema BuildStep`
    buildSteps?: [BuildStep]
    sources?: {str:Source}
    # 额外附加信息，动态表转静态表阶段使用，不存在于静态表。
    options?: {str:any}
    # 额外附加信息，静态表填充模板阶段使用，存在于静态表。
    extra?: {str:any}
